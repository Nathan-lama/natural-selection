use crate::*;
use crate::map::*;
use bevy::{
    prelude::*,
    time::common_conditions::on_timer,
};
use std::time::Duration;

pub struct EnvironmentPlugin;

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct Obstacle;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapGrid::default())
            .add_systems(Startup, setup_environment)
            .add_systems(
                Update,
                spawn_food.run_if(on_timer(Duration::from_secs_f32(FOOD_SPAWN_RATE))),
            );
    }
}

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut grid: ResMut<MapGrid>,
) {
    // Générer les obstacles via la grille procédurale
    generate_obstacles(&mut commands, &mut meshes, &mut materials, &mut grid, NUM_OBSTACLES as usize);
}

fn spawn_food(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut grid: ResMut<MapGrid>,
) {
    // Obtenir une position libre pour la nourriture
    if let Some((x, z)) = grid.get_random_free_cell() {
        // Le sol est à Y = -1.0, donc Y = -0.7 pour que la sphère touche le sol
        let pos = MapGrid::cell_to_world(x, z) + Vec3::Y * -0.7;
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.3, ..default() })),
                material: materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    emissive: Color::GREEN * 0.2,
                    ..default()
                }),
                transform: Transform::from_translation(pos),
                ..default()
            },
            Food,
        ));
    }
} 