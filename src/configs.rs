// Global
pub const W: f32 = 800.0;
pub const H: f32 = 600.0;
pub const BG_COLOR: (u8, u8, u8) = (0, 0, 0);

// Créatures évolutives
pub const NUM_CREATURES: u32 = 50;
pub const CREATURE_SPEED_MIN: f32 = 0.5;
pub const CREATURE_SPEED_MAX: f32 = 3.0;
pub const CREATURE_VISION_MIN: f32 = 20.0;
pub const CREATURE_VISION_MAX: f32 = 100.0;
pub const CREATURE_ENERGY_MAX: f32 = 100.0;
pub const CREATURE_ENERGY_CONSUMPTION_RATE: f32 = 1.0;
pub const CREATURE_ENERGY_FROM_FOOD: f32 = 20.0;
pub const CREATURE_REPRODUCTION_ENERGY_THRESHOLD: f32 = 90.0;
pub const CREATURE_REPRODUCTION_COST: f32 = 60.0;
pub const CREATURE_LIFESPAN_MAX: f32 = 120.0; // secondes
pub const CREATURE_SPRITE_SCALE: f32 = 1.0;
pub const CREATURE_Z_INDEX: f32 = 3.0;
pub const CREATURE_DIRECTION_RANDOMNESS_DEG: f32 = 300.0;
pub const CREATURE_DIRECTION_UPDATE_INTERVAL: f32 = 0.5;
pub const CREATURE_STEERING_FORCE_FACTOR: f32 = 0.7;

// Mutation
pub const MUTATION_RATE: f32 = 0.1;
pub const MUTATION_STRENGTH: f32 = 0.2;

// Ressources
pub const NUM_FOOD_SOURCES: u32 = 15;
pub const FOOD_SPAWN_RATE: f32 = 3.0; // secondes
pub const FOOD_SPRITE_SCALE: f32 = 0.8;
pub const FOOD_PICKUP_RADIUS: f32 = 3.0;
pub const FOOD_SPAWN_RADIUS: f32 = 200.0;

// Obstacles
pub const NUM_OBSTACLES: u32 = 15;
pub const OBSTACLE_SPRITE_SCALE: f32 = 1.0;
pub const OBSTACLE_COLLISION_RADIUS: f32 = 25.0;

// Évolution
pub const GENERATION_TIME: f32 = 30.0; // secondes
pub const ELITE_PERCENTAGE: f32 = 0.2; // 20% des meilleurs survivent
pub const POPULATION_LIMIT: u32 = 150;

// Sprites
pub const SPRITE_CREATURE: &str = "ant.png";
pub const SPRITE_FOOD: &str = "food.png";
pub const SPRITE_OBSTACLE: &str = "nest.png"; // Réutiliser le sprite du nid pour les obstacles
