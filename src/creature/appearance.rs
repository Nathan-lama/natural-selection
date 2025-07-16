use bevy::prelude::*;
use crate::creature::{Genetics, Energy, Creature};
use crate::*;

// Constantes pour l'animation
const WALK_ANIMATION_FRAMES: [&str; 4] = [
    "ant.png",           // Position normale
    "antt.png",          // Jambe droite levée
    "ant.png",           // Position normale
    "ant_with_food.png", // Jambe gauche levée (utilise une image différente pour la variété)
];

const WALK_FRAME_DURATION: f32 = 0.2; // Durée de chaque frame en secondes

// Nouveau composant pour l'animation de marche
#[derive(Component)]
pub struct WalkAnimation {
    pub current_frame: usize,
    pub frame_timer: f32,
    pub frame_duration: f32,
    pub is_moving: bool,
}

pub fn update_creature_colors(
    mut creature_query: Query<(&Genetics, &Energy, &mut Handle<StandardMaterial>, &mut Transform), With<Creature>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (genetics, energy, material_handle, mut transform) in creature_query.iter_mut() {
        // Couleur basée sur les traits génétiques
        let speed_factor = (genetics.speed - CREATURE_SPEED_MIN) / (CREATURE_SPEED_MAX - CREATURE_SPEED_MIN);
        let vision_factor = (genetics.vision - CREATURE_VISION_MIN) / (CREATURE_VISION_MAX - CREATURE_VISION_MIN);
        let energy_factor = energy.0 / CREATURE_ENERGY_MAX;
        
        // Rouge = vitesse, Vert = vision, Bleu = efficacité énergétique
        if let Some(material) = materials.get_mut(&material_handle) {
            material.base_color = Color::rgb(
                speed_factor.clamp(0.0, 1.0),
                vision_factor.clamp(0.0, 1.0),
                (genetics.energy_efficiency / 1.5).clamp(0.0, 1.0)
            );
        }
        
        // Taille basée sur l'énergie (échelle 3D)
        let min_scale = 0.3;
        let max_scale = 0.8;
        let scale = min_scale + (max_scale - min_scale) * energy_factor.clamp(0.0, 1.0);
        transform.scale = Vec3::splat(scale);
    }
}

pub fn update_walk_animation(
    mut creature_query: Query<(&mut WalkAnimation, &crate::creature::movement::Velocity, &mut Transform), With<Creature>>,
    time: Res<Time>,
) {
    for (mut walk_animation, velocity, mut transform) in creature_query.iter_mut() {
        walk_animation.frame_timer += time.delta_seconds();

        if velocity.0.length_squared() > 0.1 { // Only animate if moving
            walk_animation.is_moving = true;
            if walk_animation.frame_timer >= walk_animation.frame_duration {
                walk_animation.current_frame = (walk_animation.current_frame + 1) % WALK_ANIMATION_FRAMES.len();
                walk_animation.frame_timer = 0.0;
                
                // Animation 3D simple : faire rebondir légèrement la créature
                let bounce_height = 0.2;
                transform.translation.y = if walk_animation.current_frame % 2 == 0 {
                    bounce_height
                } else {
                    0.0
                };
            }
        } else {
            walk_animation.is_moving = false;
            walk_animation.current_frame = 0; // Reset to idle frame
            // Revenir à la position normale quand immobile
            transform.translation.y = 0.0;
        }
    }
} 