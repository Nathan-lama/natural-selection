use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

pub const MAP_WIDTH: i32 = 60; // X: -30 à 30
pub const MAP_HEIGHT: i32 = 40; // Z: -20 à 20
pub const CELL_SIZE: f32 = 2.0; // Taille logique d'une case

#[derive(Default, Resource)]
pub struct MapGrid {
    pub occupied: HashSet<(i32, i32)>, // (x, z) des cases occupées par obstacles
}

impl MapGrid {
    pub fn is_occupied(&self, x: i32, z: i32) -> bool {
        self.occupied.contains(&(x, z))
    }
    pub fn mark_occupied(&mut self, x: i32, z: i32) {
        self.occupied.insert((x, z));
    }
    pub fn get_random_free_cell(&self) -> Option<(i32, i32)> {
        let mut rng = thread_rng();
        let mut tries = 0;
        while tries < 100 {
            let x = rng.gen_range(-MAP_WIDTH/2..MAP_WIDTH/2);
            let z = rng.gen_range(-MAP_HEIGHT/2..MAP_HEIGHT/2);
            if !self.is_occupied(x, z) {
                return Some((x, z));
            }
            tries += 1;
        }
        None
    }
    pub fn cell_to_world(x: i32, z: i32) -> Vec3 {
        Vec3::new(x as f32 * CELL_SIZE, 0.0, z as f32 * CELL_SIZE)
    }
}

pub fn generate_obstacles(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, grid: &mut MapGrid, num_obstacles: usize) {
    let mut rng = thread_rng();
    let mut placed = 0;
    while placed < num_obstacles {
        let x = rng.gen_range(-MAP_WIDTH/2..MAP_WIDTH/2);
        let z = rng.gen_range(-MAP_HEIGHT/2..MAP_HEIGHT/2);
        if grid.is_occupied(x, z) { continue; }
        grid.mark_occupied(x, z);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: 1.0,
                    height: 4.0,
                    resolution: 8,
                    segments: 1,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.5, 0.5, 0.5),
                    perceptual_roughness: 0.8,
                    ..default()
                }),
                // Le sol est à Y = -1.0, donc Y = 0.0 pour que la base du cylindre touche le sol
                transform: Transform::from_translation(MapGrid::cell_to_world(x, z) + Vec3::Y * 0.0),
                ..Default::default()
            },
        ));
        placed += 1;
    }
} 