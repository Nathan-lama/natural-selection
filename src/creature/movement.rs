use bevy::prelude::*;
use bevy::math::{vec3, Mat2, Vec3Swizzles};
use rand::{thread_rng, Rng};
use crate::creature::{Creature, Genetics};
use crate::*;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Acceleration(pub Vec3);

#[derive(Component)]
pub struct WanderTimer {
    pub timer: f32,
    pub interval: f32,
}

pub fn wander_direction_system(
    mut query: Query<(&mut Velocity, &mut WanderTimer, &Transform, &Genetics), With<Creature>>,
    obstacle_query: Query<&Transform, (With<crate::environment::Obstacle>, Without<Creature>)>,
    time: Res<Time>,
) {
    for (mut velocity, mut wander, transform, genetics) in query.iter_mut() {
        wander.timer += time.delta_seconds();
        
        // Changement de direction plus fréquent et plus naturel
        if wander.timer > wander.interval {
            let mut rng = thread_rng();
            
            // Rotation plus importante pour un mouvement plus naturel
            let angle = rng.gen_range(-2.0..2.0); // rotation plus importante
            
            // Appliquer la rotation à la vélocité actuelle
            let current_direction = velocity.0.normalize();
            let rotation_matrix = Mat2::from_angle(angle);
            let rotated_direction = rotation_matrix * current_direction.xz();
            
            // Nouvelle vélocité avec direction aléatoire
            velocity.0 = vec3(rotated_direction.x, 0.0, rotated_direction.y) * genetics.speed;
            
            // Réinitialiser le timer avec un intervalle plus court
            wander.timer = 0.0;
            wander.interval = rng.gen_range(0.8..2.0); // intervalle plus court
        }
        
        // Bruit aléatoire constant pour éviter le mouvement linéaire
        let mut rng = thread_rng();
        if rng.gen_bool(0.15) { // 15% de chance par frame
            let noise_angle = rng.gen_range(-0.3..0.3);
            let current_direction = velocity.0.normalize();
            let rotation_matrix = Mat2::from_angle(noise_angle);
            let rotated_direction = rotation_matrix * current_direction.xz();
            velocity.0 = vec3(rotated_direction.x, 0.0, rotated_direction.y) * genetics.speed;
        }
        
        // Évitement des bords de la map
        let pos = transform.translation;
        let margin = 3.0;
        let mut avoid_force = Vec3::ZERO;
        
        if pos.x < -30.0 + margin {
            avoid_force.x += 2.0;
        } else if pos.x > 30.0 - margin {
            avoid_force.x -= 2.0;
        }
        
        if pos.z < -20.0 + margin {
            avoid_force.z += 2.0;
        } else if pos.z > 20.0 - margin {
            avoid_force.z -= 2.0;
        }
        
        // Appliquer la force d'évitement
        if avoid_force.length() > 0.0 {
            velocity.0 += avoid_force * time.delta_seconds();
        }
        
        // Évitement des obstacles
        for obstacle_transform in obstacle_query.iter() {
            let obstacle_pos = obstacle_transform.translation;
            let distance = (pos - obstacle_pos).length();
            
            if distance < 5.0 && distance > 0.0 {
                let avoid_direction = (pos - obstacle_pos).normalize();
                let avoid_strength = 3.0 / distance;
                velocity.0 += avoid_direction * avoid_strength * time.delta_seconds();
            }
        }
        
        // Limiter la vitesse maximale
        if velocity.0.length() > genetics.speed {
            velocity.0 = velocity.0.normalize() * genetics.speed;
        }
    }
}

pub fn update_position(
    mut creature_query: Query<(&mut Transform, &mut Velocity, &mut Acceleration, &Genetics), With<Creature>>,
    time: Res<Time>,
    settings: Res<crate::gui::SimSettings>,
) {
    let speed_factor = settings.simulation_speed;
    for (mut transform, mut velocity, mut acceleration, genetics) in creature_query.iter_mut() {
        // Combiner la vélocité du random walk avec l'accélération vers la nourriture
        velocity.0 += acceleration.0 * time.delta_seconds() * speed_factor;
        
        // Limiter la vitesse maximale
        let max_speed = genetics.speed;
        if velocity.0.length() > max_speed {
            velocity.0 = velocity.0.normalize() * max_speed;
        }
        
        // Appliquer la vélocité à la position
        transform.translation += velocity.0 * time.delta_seconds() * speed_factor;
        
        // Mettre à jour la rotation pour qu'elle suive la direction du mouvement
        if velocity.0.length() > 0.1 {
            let direction = velocity.0.normalize();
            let angle = direction.x.atan2(direction.z);
            transform.rotation = Quat::from_rotation_y(-angle);
        }
        
        // Réinitialiser l'accélération pour le prochain frame
        acceleration.0 = Vec3::ZERO;
    }
} 