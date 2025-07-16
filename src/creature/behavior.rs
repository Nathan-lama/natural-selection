use bevy::prelude::*;
use bevy::math::{vec2, vec3, Vec3Swizzles};
use rand::{thread_rng, Rng};
use crate::creature::{Acceleration, Genetics, Creature, Energy, Fitness, Age, Generation, EvolutionStats};
use crate::environment::{Food, Obstacle};
use crate::utils::get_rand_unit_vec2;
use crate::*;

#[derive(Component)]
pub struct Target(pub Option<Vec3>);

pub fn update_creature_behavior(
    mut creature_query: Query<(
        &mut Acceleration,
        &Transform,
        &Genetics,
        &mut Target,
        Entity,
    ), With<Creature>>,
    food_query: Query<&Transform, (With<Food>, Without<Creature>)>,
    _obstacle_query: Query<&Transform, (With<Obstacle>, Without<Creature>)>,
    _time: Res<Time>,
    settings: Res<crate::gui::SimSettings>,
) {
    let _speed_factor = settings.simulation_speed;
    for (mut acceleration, transform, genetics, mut target, _entity) in creature_query.iter_mut() {
        let current_pos = transform.translation;
        
        // Chercher de la nourriture dans le rayon de vision
        let mut nearest_food = None;
        let mut nearest_distance = f32::INFINITY;
        
        for food_transform in food_query.iter() {
            let distance = (current_pos - food_transform.translation).length();
            if distance < genetics.vision && distance < nearest_distance {
                nearest_food = Some(food_transform.translation);
                nearest_distance = distance;
            }
        }
        
        // Si de la nourriture est trouvée, se diriger vers elle
        if let Some(food_pos) = nearest_food {
            target.0 = Some(food_pos);
            let direction = (food_pos - current_pos).normalize();
            acceleration.0 = direction * genetics.speed * 0.5; // Force d'attraction modérée
        } else {
            // Pas de nourriture en vue, laisser le random walk faire son travail
            target.0 = None;
            acceleration.0 = Vec3::ZERO;
        }
    }
}

pub fn update_energy_and_age(
    mut creature_query: Query<(&mut Energy, &mut Age, &Genetics), With<Creature>>,
    time: Res<Time>,
    settings: Res<crate::gui::SimSettings>,
) {
    let speed_factor = settings.simulation_speed;
    for (mut energy, mut age, genetics) in creature_query.iter_mut() {
        // Consommation d'énergie basée sur l'efficacité
        let consumption = CREATURE_ENERGY_CONSUMPTION_RATE / genetics.energy_efficiency;
        energy.0 = f32::max(energy.0 - consumption * time.delta_seconds() * speed_factor, 0.0);
        
        // Vieillissement
        age.0 += time.delta_seconds() * speed_factor;
    }
}

pub fn check_creature_death(
    mut commands: Commands,
    creature_query: Query<(Entity, &Energy, &Age), With<Creature>>,
) {
    for (entity, energy, age) in creature_query.iter() {
        // Mort par manque d'énergie ou par âge
        if energy.0 <= 0.0 || age.0 >= CREATURE_LIFESPAN_MAX {
            commands.entity(entity).despawn();
        }
    }
}

pub fn check_food_collisions(
    mut creature_query: Query<(&Transform, &mut Energy, &mut Fitness), With<Creature>>,
    food_query: Query<(Entity, &Transform), With<Food>>,
    mut commands: Commands,
) {
    let mut food_to_remove = Vec::new();
    for (creature_transform, mut energy, mut fitness) in creature_query.iter_mut() {
        for (food_entity, food_transform) in food_query.iter() {
            // Calculer la distance seulement en X et Z (ignorer Y)
            let creature_pos_2d = vec2(creature_transform.translation.x, creature_transform.translation.z);
            let food_pos_2d = vec2(food_transform.translation.x, food_transform.translation.z);
            let distance_2d = creature_pos_2d.distance(food_pos_2d);
            
            if distance_2d <= FOOD_PICKUP_RADIUS {
                energy.0 = f32::min(energy.0 + CREATURE_ENERGY_FROM_FOOD, CREATURE_ENERGY_MAX);
                fitness.0 += 10.0;
                food_to_remove.push(food_entity);
            }
        }
    }
    for food_entity in food_to_remove {
        commands.entity(food_entity).despawn();
    }
}

pub fn check_obstacle_collisions(
    mut creature_query: Query<(&Transform, &mut crate::creature::movement::Velocity), With<Creature>>,
    obstacle_query: Query<&Transform, With<Obstacle>>,
) {
    for (creature_transform, mut velocity) in creature_query.iter_mut() {
        for obstacle_transform in obstacle_query.iter() {
            let distance = (creature_transform.translation.xz() - obstacle_transform.translation.xz()).length();
            if distance <= OBSTACLE_COLLISION_RADIUS {
                let normal = (creature_transform.translation - obstacle_transform.translation).normalize();
                velocity.0 = velocity.0 - 2.0 * velocity.0.dot(normal) * normal;
            }
        }
    }
}

pub fn reproduction_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    creature_query: Query<(
        Entity,
        &Transform,
        &Genetics,
        &Energy,
        &Fitness,
    ), With<Creature>>,
) {
    let mut creatures_to_reproduce = Vec::new();
    for (entity, transform, genetics, energy, fitness) in creature_query.iter() {
        if energy.0 >= CREATURE_REPRODUCTION_ENERGY_THRESHOLD && fitness.0 > 30.0 {
            // Limiter la reproduction : seulement 20% de chance (moins de surpopulation)
            if thread_rng().gen_bool(0.2) {
                creatures_to_reproduce.push((entity, transform.clone(), genetics.clone()));
            }
        }
    }
    
    // Limiter le nombre de reproductions simultanées pour éviter le clignotement
    let max_reproductions = 5;
    if creatures_to_reproduce.len() > max_reproductions {
        creatures_to_reproduce.truncate(max_reproductions);
    }
    
    // Collecter les entités des parents pour réduire leur énergie
    let mut parent_entities = Vec::new();
    
    for (parent_entity, parent_transform, parent_genetics) in &creatures_to_reproduce {
        let child_genetics = crate::creature::genetics::mutate_genetics(parent_genetics);
        let offset = get_rand_unit_vec2() * 8.0; // Distance plus petite
        let child_position = parent_transform.translation + vec3(offset.x, 0.0, offset.y);
        // S'assurer que l'enfant reste dans les limites
        let clamped_x = child_position.x.clamp(-35.0, 35.0);
        let clamped_z = child_position.z.clamp(-25.0, 25.0);
        let final_position = vec3(clamped_x, 1.0, clamped_z);
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 0.8,
                    depth: 1.6,
                    ..default()
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(1.0, 0.5, 0.0),
                    perceptual_roughness: 0.3,
                    metallic: 0.1,
                    ..default()
                }),
                transform: Transform::from_xyz(
                    final_position.x,
                    final_position.y,
                    final_position.z,
                ).with_scale(Vec3::splat(CREATURE_SPRITE_SCALE)),
                ..Default::default()
            },
            Creature,
            child_genetics,
            Energy(CREATURE_ENERGY_MAX * 0.6), // Plus d'énergie pour les enfants
            Age(0.0),
            Fitness(0.0),
            crate::creature::movement::Velocity(get_rand_unit_vec2().extend(0.0)),
            crate::creature::movement::Acceleration(Vec3::ZERO),
            Target(None),
            crate::creature::appearance::WalkAnimation {
                current_frame: 0,
                frame_timer: 0.0,
                frame_duration: 0.2,
                is_moving: false,
            },
            crate::creature::movement::WanderTimer {
                timer: 0.0,
                interval: thread_rng().gen_range(1.5..3.5),
            },
        ));
        parent_entities.push(*parent_entity);
    }
    
    // Réduire l'énergie du parent APRÈS la boucle
    for parent_entity in parent_entities {
        commands.entity(parent_entity).insert(Energy(CREATURE_REPRODUCTION_COST));
    }
}

pub fn evolution_generation(
    mut commands: Commands,
    mut generation: ResMut<Generation>,
    mut evolution_stats: ResMut<EvolutionStats>,
    creature_query: Query<(Entity, &Fitness, &Genetics, &Age), With<Creature>>,
) {
    let mut creatures: Vec<(Entity, f32, Genetics, f32)> = creature_query
        .iter()
        .map(|(entity, fitness, genetics, age)| (entity, fitness.0, genetics.clone(), age.0))
        .collect();
    
    // Trier par fitness (meilleur en premier)
    creatures.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    let total_creatures = creatures.len();
    
    // Évolution plus douce : seulement supprimer les 10% les plus faibles
    let elite_count = (total_creatures as f32 * 0.9) as usize; // 90% survivent
    
    // Supprimer progressivement les créatures les moins performantes
    let creatures_to_remove = creatures.iter().skip(elite_count).take(5); // Max 5 créatures à la fois
    for (entity, _, _, _) in creatures_to_remove {
        commands.entity(*entity).despawn();
    }
    
    // Mettre à jour les statistiques
    generation.0 += 1;
    evolution_stats.generation = generation.0;
    evolution_stats.total_creatures = total_creatures;
    
    if !creatures.is_empty() {
        let avg_fitness = creatures.iter().map(|(_, fitness, _, _)| *fitness).sum::<f32>() / total_creatures as f32;
        let best_fitness = creatures[0].1;
        let avg_speed = creatures.iter().map(|(_, _, genetics, _)| genetics.speed).sum::<f32>() / total_creatures as f32;
        let avg_vision = creatures.iter().map(|(_, _, genetics, _)| genetics.vision).sum::<f32>() / total_creatures as f32;
        
        evolution_stats.avg_fitness = avg_fitness;
        evolution_stats.best_fitness = best_fitness;
        evolution_stats.avg_speed = avg_speed;
        evolution_stats.avg_vision = avg_vision;
    }
}

pub fn update_evolution_stats(
    mut evolution_stats: ResMut<EvolutionStats>,
    creature_query: Query<(&Fitness, &Genetics), With<Creature>>,
) {
    let creatures: Vec<_> = creature_query.iter().collect();
    let total_creatures = creatures.len();
    
    if total_creatures > 0 {
        let avg_fitness = creatures.iter().map(|(fitness, _)| fitness.0).sum::<f32>() / total_creatures as f32;
        let best_fitness = creatures.iter().map(|(fitness, _)| fitness.0).fold(0.0, f32::max);
        let avg_speed = creatures.iter().map(|(_, genetics)| genetics.speed).sum::<f32>() / total_creatures as f32;
        let avg_vision = creatures.iter().map(|(_, genetics)| genetics.vision).sum::<f32>() / total_creatures as f32;
        
        evolution_stats.total_creatures = total_creatures;
        evolution_stats.avg_fitness = avg_fitness;
        evolution_stats.best_fitness = best_fitness;
        evolution_stats.avg_speed = avg_speed;
        evolution_stats.avg_vision = avg_vision;
    }
} 