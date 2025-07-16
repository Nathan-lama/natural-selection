use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;
use rand::{thread_rng, Rng};
use crate::creature::*;
use crate::utils::get_rand_unit_vec2;
use crate::*;

pub struct CreaturePlugin;

#[derive(Component)]
pub struct Creature;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .insert_resource(Generation(1))
            .insert_resource(EvolutionStats::default())
            .add_systems(
                Update,
                behavior::update_creature_behavior.run_if(on_timer(Duration::from_secs_f32(0.5))),
            )
            .add_systems(Update, behavior::update_energy_and_age)
            .add_systems(Update, behavior::check_creature_death)
            .add_systems(Update, behavior::check_food_collisions)
            .add_systems(Update, behavior::check_obstacle_collisions)
            .add_systems(Update, movement::update_position.after(behavior::check_obstacle_collisions))
            .add_systems(Update, appearance::update_creature_colors)
            .add_systems(Update, appearance::update_walk_animation)
            .add_systems(Update, behavior::reproduction_system)
            .add_systems(
                Update,
                behavior::evolution_generation.run_if(on_timer(Duration::from_secs_f32(60.0))),
            )
            .add_systems(
                Update,
                behavior::update_evolution_stats.run_if(on_timer(Duration::from_secs_f32(5.0))),
            )
            .add_systems(Update, movement::wander_direction_system);
    }
}

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    for _ in 0..NUM_CREATURES {
        let genetics = genetics::generate_random_genetics();
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 1.0,
                    depth: 2.0,
                    ..default()
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(1.0, 0.5, 0.0),
                    perceptual_roughness: 0.3,
                    metallic: 0.1,
                    ..default()
                }),
                transform: Transform::from_xyz(
                    thread_rng().gen_range(-30.0..30.0),
                    1.0,
                    thread_rng().gen_range(-20.0..20.0),
                ).with_scale(Vec3::splat(CREATURE_SPRITE_SCALE)),
                ..Default::default()
            },
            Creature,
            genetics,
            Energy(CREATURE_ENERGY_MAX),
            Age(0.0),
            Fitness(0.0),
            movement::Velocity(get_rand_unit_vec2().extend(0.0)),
            movement::Acceleration(Vec3::ZERO),
            behavior::Target(None),
            appearance::WalkAnimation {
                current_frame: 0,
                frame_timer: 0.0,
                frame_duration: 0.2,
                is_moving: false,
            },
            movement::WanderTimer {
                timer: 0.0,
                interval: thread_rng().gen_range(1.5..3.5),
            },
        ));
    }
} 