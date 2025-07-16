use crate::creature::{Creature, EvolutionStats, Generation, Genetics, Fitness};
use crate::environment::{Food, Obstacle};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct GuiPlugin;

#[derive(Resource)]
pub struct SimSettings {
    pub is_show_creatures: bool,
    pub is_show_food: bool,
    pub is_show_obstacles: bool,
    pub is_show_menu: bool,
    pub simulation_speed: f32,
    pub show_graphs: bool,
}

#[derive(Default, Resource)]
pub struct SimStatistics {
    pub num_creatures: usize,
    pub num_food: usize,
    pub num_obstacles: usize,
    pub speed_history: Vec<f32>,
    pub vision_history: Vec<f32>,
    pub fitness_history: Vec<f32>,
}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(SimSettings::default())
            .insert_resource(SimStatistics::default())
            .add_systems(Update, settings_dialog)
            .add_systems(Update, settings_toggle)
            .add_systems(Update, update_sim_stats)
            .add_plugins(EguiPlugin)
            .add_systems(Startup, setup);
    }
}

fn setup() {}

fn settings_toggle(
    mut settings: ResMut<SimSettings>,
    keys: Res<Input<KeyCode>>,
    mut params: ParamSet<(
        Query<&mut Visibility, With<Creature>>,
        Query<&mut Visibility, With<Food>>,
        Query<&mut Visibility, With<Obstacle>>,
    )>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        settings.is_show_menu = !settings.is_show_menu;
    }
    if keys.just_pressed(KeyCode::C) {
        settings.is_show_creatures = !settings.is_show_creatures;
        for mut creature in params.p0().iter_mut() {
            if settings.is_show_creatures {
                *creature = Visibility::Visible;
            } else {
                *creature = Visibility::Hidden;
            }
        }
    }
    if keys.just_pressed(KeyCode::F) {
        settings.is_show_food = !settings.is_show_food;
        for mut food in params.p1().iter_mut() {
            if settings.is_show_food {
                *food = Visibility::Visible;
            } else {
                *food = Visibility::Hidden;
            }
        }
    }
    if keys.just_pressed(KeyCode::O) {
        settings.is_show_obstacles = !settings.is_show_obstacles;
        for mut obstacle in params.p2().iter_mut() {
            if settings.is_show_obstacles {
                *obstacle = Visibility::Visible;
            } else {
                *obstacle = Visibility::Hidden;
            }
        }
    }
}

fn settings_dialog(
    mut contexts: EguiContexts,
    mut settings: ResMut<SimSettings>,
    stats: Res<SimStatistics>,
    evolution_stats: Res<EvolutionStats>,
    generation: Res<Generation>,
    creature_query: Query<&Genetics, With<Creature>>,
) {
    if !settings.is_show_menu {
        return;
    }

    let ctx = contexts.ctx_mut();

    egui::Window::new("Simulation d'Évolution")
        .title_bar(true)
        .default_pos(egui::pos2(10.0, 10.0))
        .show(ctx, |ui| {
            egui::CollapsingHeader::new("Statistiques d'Évolution")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label(format!("Génération: {}", generation.0));
                    ui.label(format!("Créatures: {}", stats.num_creatures));
                    ui.label(format!("Nourriture: {}", stats.num_food));
                    ui.label(format!("Obstacles: {}", stats.num_obstacles));
                    ui.separator();
                    ui.label(format!("Fitness moyen: {:.1}", evolution_stats.avg_fitness));
                    ui.label(format!("Meilleur fitness: {:.1}", evolution_stats.best_fitness));
                    ui.label(format!("Vitesse moyenne: {:.2}", evolution_stats.avg_speed));
                    ui.label(format!("Vision moyenne: {:.1}", evolution_stats.avg_vision));
                });
            
            egui::CollapsingHeader::new("Graphiques d'Évolution")
                .default_open(false)
                .show(ui, |ui| {
                    ui.checkbox(&mut settings.show_graphs, "Afficher les graphiques");
                    
                    if settings.show_graphs && !stats.speed_history.is_empty() {
                        // Graphique simple de l'évolution de la vitesse
                        ui.label("Évolution de la vitesse moyenne:");
                        
                        // Créer un graphique simple avec des barres
                        let max_speed = stats.speed_history.iter().fold(0.0_f32, |a, &b| a.max(b));
                        let min_speed = stats.speed_history.iter().fold(f32::INFINITY, |a, &b| a.min(b));
                        
                        ui.label(format!("Vitesse actuelle: {:.2}", stats.speed_history.last().unwrap_or(&0.0)));
                        ui.label(format!("Vitesse max: {:.2}", max_speed));
                        ui.label(format!("Vitesse min: {:.2}", min_speed));
                        
                        // Barre de progression simple
                        if let Some(current_speed) = stats.speed_history.last() {
                            let progress = (current_speed - min_speed) / (max_speed - min_speed).max(0.001);
                            ui.add(egui::ProgressBar::new(progress.clamp(0.0, 1.0))
                                .text(format!("{:.2}", current_speed)));
                        }
                        
                        // Distribution des traits actuels
                        ui.separator();
                        ui.label("Distribution des traits actuels:");
                        let mut speed_values: Vec<f32> = creature_query
                            .iter()
                            .map(|genetics| genetics.speed)
                            .collect();
                        speed_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        
                        if !speed_values.is_empty() {
                            let min_speed = speed_values[0];
                            let max_speed = speed_values[speed_values.len() - 1];
                            let avg_speed = speed_values.iter().sum::<f32>() / speed_values.len() as f32;
                            
                            ui.label(format!("Vitesse - Min: {:.2}, Max: {:.2}, Moy: {:.2}", 
                                min_speed, max_speed, avg_speed));
                            
                            // Histogramme simple
                            let mut vision_values: Vec<f32> = creature_query
                                .iter()
                                .map(|genetics| genetics.vision)
                                .collect();
                            vision_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
                            
                            if !vision_values.is_empty() {
                                let min_vision = vision_values[0];
                                let max_vision = vision_values[vision_values.len() - 1];
                                let avg_vision = vision_values.iter().sum::<f32>() / vision_values.len() as f32;
                                
                                ui.label(format!("Vision - Min: {:.1}, Max: {:.1}, Moy: {:.1}", 
                                    min_vision, max_vision, avg_vision));
                            }
                        }
                    }
                });
            
            egui::CollapsingHeader::new("Paramètres")
                .default_open(true)
                .show(ui, |ui| {
                    ui.checkbox(&mut settings.is_show_creatures, "Afficher créatures (C)");
                    ui.checkbox(&mut settings.is_show_food, "Afficher nourriture (F)");
                    ui.checkbox(&mut settings.is_show_obstacles, "Afficher obstacles (O)");
                    ui.separator();
                    ui.label("Vitesse de simulation:");
                    ui.add(egui::Slider::new(&mut settings.simulation_speed, 0.1..=100.0));
                });
            
            egui::CollapsingHeader::new("Légende des Couleurs")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label("🔴 Rouge = Vitesse (plus rouge = plus rapide)");
                    ui.label("🟢 Vert = Vision (plus vert = meilleure vision)");
                    ui.label("🔵 Bleu = Efficacité énergétique");
                    ui.label("📏 Taille = Niveau d'énergie (plus grand = plus d'énergie)");
                });
            
            egui::CollapsingHeader::new("Contrôles")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label("🎮 Contrôles de la caméra:");
                    ui.label("• Clic gauche + glisser = Déplacer la caméra");
                    ui.label("• Molette de souris = Zoom avant/arrière");
                    ui.label("• Clic milieu + glisser = Rotation de la caméra");
                    ui.separator();
                    ui.label("📷 Changement de vue:");
                    ui.label("• Touche 1 = Vue top-down (vue d'ensemble)");
                    ui.label("• Touche 2 = Vue isométrique (vue 3D)");
                    ui.label("• Touche 3 = Vue de côté");
                    ui.label("• Touche 4 = Vue rapprochée");
                    ui.label("• Touche 5 = Vue éloignée");
                    ui.separator();
                    ui.label("🎯 Autres contrôles:");
                    ui.label("• Tab = Afficher/masquer ce menu");
                    ui.label("• C = Afficher/masquer les créatures");
                    ui.label("• F = Afficher/masquer la nourriture");
                    ui.label("• O = Afficher/masquer les obstacles");
                    ui.label("• Échap = Quitter");
                    ui.label("Tab: Menu");
                    ui.label("C: Créatures");
                    ui.label("F: Nourriture");
                    ui.label("O: Obstacles");
                    ui.label("Échap: Quitter");
                });
        });
}

fn update_sim_stats(
    mut stats: ResMut<SimStatistics>,
    creature_query: Query<(&Genetics, &Fitness), With<Creature>>,
    food_query: Query<With<Food>>,
    obstacle_query: Query<With<Obstacle>>,
    _evolution_stats: Res<EvolutionStats>,
) {
    stats.num_creatures = creature_query.iter().len();
    stats.num_food = food_query.iter().len();
    stats.num_obstacles = obstacle_query.iter().len();
    
    // Collecter l'historique des traits pour les graphiques
    if stats.num_creatures > 0 {
        let avg_speed = creature_query.iter()
            .map(|(genetics, _)| genetics.speed)
            .sum::<f32>() / stats.num_creatures as f32;
        
        let avg_vision = creature_query.iter()
            .map(|(genetics, _)| genetics.vision)
            .sum::<f32>() / stats.num_creatures as f32;
        
        let avg_fitness = creature_query.iter()
            .map(|(_, fitness)| fitness.0)
            .sum::<f32>() / stats.num_creatures as f32;
        
        // Garder seulement les 100 dernières valeurs pour éviter la surcharge mémoire
        stats.speed_history.push(avg_speed);
        stats.vision_history.push(avg_vision);
        stats.fitness_history.push(avg_fitness);
        
        if stats.speed_history.len() > 100 {
            stats.speed_history.remove(0);
            stats.vision_history.remove(0);
            stats.fitness_history.remove(0);
        }
    }
}

impl Default for SimSettings {
    fn default() -> Self {
        Self {
            is_show_creatures: true,
            is_show_food: true,
            is_show_obstacles: true,
            is_show_menu: false,
            simulation_speed: 1.0,
            show_graphs: false,
        }
    }
}
