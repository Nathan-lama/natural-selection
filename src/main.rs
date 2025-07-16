use ants::{
    creature::CreaturePlugin,
    environment::EnvironmentPlugin,
    gui::{GuiPlugin, SimSettings},
    *,
};
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_pancam::{PanCam, PanCamPlugin};

#[derive(Component)]
struct FollowCamera;

#[derive(Component)]
struct CameraController;

#[derive(Resource)]
struct CameraMode {
    current: u8,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        // mode: WindowMode::Fullscreen,
                        focused: true,
                        resolution: (W, H).into(),
                        title: "Simulation d'Évolution 3D".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        // External plugins & systems
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(PanCamPlugin)
        // Default Resources
        .insert_resource(ClearColor(Color::rgba_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 0,
        )))
        .insert_resource(Msaa::Off)
        .insert_resource(CameraMode { current: 0 })
        // Systems
        .add_systems(Startup, setup)
        .add_systems(Update, update_simulation_speed)
        .add_systems(Update, camera_controls)
        // Internal Plugins
        .add_plugins(CreaturePlugin)
        .add_plugins(EnvironmentPlugin)
        .add_plugins(GuiPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Caméra 3D avec vue top-down pour voir toute la map
    commands
        .spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 50.0, 0.0)
                    .looking_at(Vec3::ZERO, Vec3::NEG_Z),
                tonemapping: Tonemapping::TonyMcMapface,
                ..default()
            },
            BloomSettings::default(),
            FollowCamera,
            CameraController,
        ))
        .insert(PanCam {
            grab_buttons: vec![MouseButton::Left, MouseButton::Middle],
            enabled: true,
            zoom_to_cursor: true,
            min_scale: 0.1,
            max_scale: Some(10.0),
            ..default()
        });

    // Éclairage 3D
    // Lumière ambiante pour l'éclairage général
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });

    // Lumière directionnelle principale (comme le soleil)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 20.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Sol 3D pour donner une référence visuelle
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 80.0, subdivisions: 0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.4, 0.2),
            perceptual_roughness: 0.8,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, -1.0, 0.0),
        ..default()
    });
}

fn update_simulation_speed(
    _sim_settings: Res<SimSettings>,
    _time: ResMut<Time>,
) {
    // Note: Bevy 0.11 doesn't have Virtual time, so we'll skip this for now
    // time.set_relative_speed(sim_settings.simulation_speed);
}

fn camera_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<CameraController>>,
    mut camera_mode: ResMut<CameraMode>,
) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        camera_mode.current = 0; // Vue top-down
    } else if keyboard_input.just_pressed(KeyCode::Key2) {
        camera_mode.current = 1; // Vue isométrique
    } else if keyboard_input.just_pressed(KeyCode::Key3) {
        camera_mode.current = 2; // Vue de côté
    } else if keyboard_input.just_pressed(KeyCode::Key4) {
        camera_mode.current = 3; // Vue rapprochée
    } else if keyboard_input.just_pressed(KeyCode::Key5) {
        camera_mode.current = 4; // Vue éloignée
    }

    if let Ok(mut transform) = camera_query.get_single_mut() {
        match camera_mode.current {
            0 => {
                // Vue top-down
                transform.translation = Vec3::new(0.0, 50.0, 0.0);
                transform.look_at(Vec3::ZERO, Vec3::NEG_Z);
            }
            1 => {
                // Vue isométrique
                transform.translation = Vec3::new(40.0, 40.0, 40.0);
                transform.look_at(Vec3::ZERO, Vec3::Y);
            }
            2 => {
                // Vue de côté
                transform.translation = Vec3::new(0.0, 20.0, 60.0);
                transform.look_at(Vec3::ZERO, Vec3::Y);
            }
            3 => {
                // Vue rapprochée
                transform.translation = Vec3::new(0.0, 15.0, 30.0);
                transform.look_at(Vec3::ZERO, Vec3::Y);
            }
            4 => {
                // Vue éloignée
                transform.translation = Vec3::new(0.0, 80.0, 0.0);
                transform.look_at(Vec3::ZERO, Vec3::NEG_Z);
            }
            _ => {}
        }
    }
}
