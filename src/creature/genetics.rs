use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::*;

#[derive(Component, Clone)]
pub struct Genetics {
    pub speed: f32,
    pub vision: f32,
    pub energy_efficiency: f32,
    pub reproduction_rate: f32,
}

#[derive(Component)]
pub struct Energy(pub f32);

#[derive(Component)]
pub struct Age(pub f32);

#[derive(Component)]
pub struct Fitness(pub f32);

#[derive(Resource)]
pub struct Generation(pub u32);

#[derive(Resource)]
pub struct EvolutionStats {
    pub generation: u32,
    pub total_creatures: usize,
    pub avg_fitness: f32,
    pub best_fitness: f32,
    pub avg_speed: f32,
    pub avg_vision: f32,
}

pub fn generate_random_genetics() -> Genetics {
    let mut rng = thread_rng();
    Genetics {
        speed: rng.gen_range(CREATURE_SPEED_MIN..CREATURE_SPEED_MAX),
        vision: rng.gen_range(CREATURE_VISION_MIN..CREATURE_VISION_MAX),
        energy_efficiency: rng.gen_range(0.5..1.5),
        reproduction_rate: rng.gen_range(0.5..1.5),
    }
}

pub fn mutate_genetics(genetics: &Genetics) -> Genetics {
    let mut rng = thread_rng();
    let mut mutation_factor = |value: f32, min: f32, max: f32| {
        if rng.gen_bool(MUTATION_RATE as f64) {
            let mutation = rng.gen_range(-MUTATION_STRENGTH..MUTATION_STRENGTH);
            (value * (1.0 + mutation)).clamp(min, max)
        } else {
            value
        }
    };

    Genetics {
        speed: mutation_factor(genetics.speed, CREATURE_SPEED_MIN, CREATURE_SPEED_MAX),
        vision: mutation_factor(genetics.vision, CREATURE_VISION_MIN, CREATURE_VISION_MAX),
        energy_efficiency: mutation_factor(genetics.energy_efficiency, 0.5, 1.5),
        reproduction_rate: mutation_factor(genetics.reproduction_rate, 0.5, 1.5),
    }
}

impl Default for EvolutionStats {
    fn default() -> Self {
        Self {
            generation: 1,
            total_creatures: 0,
            avg_fitness: 0.0,
            best_fitness: 0.0,
            avg_speed: 0.0,
            avg_vision: 0.0,
        }
    }
} 