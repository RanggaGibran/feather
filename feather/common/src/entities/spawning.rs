use base::{BlockPosition, Biome, BiomeId, EntityKind};
use ecs::{Entity, IntoQuery, SysResult, SystemExecutor};
use rand::{Rng, thread_rng};
use quill_common::entities::{Axolotl, Goat, GlowSquid};
use std::time::Duration;

use crate::Game;

/// Represents the difficulty of spawning for an entity
enum SpawnDifficulty {
    /// Entity can always spawn regardless of difficulty
    Any,
    /// Entity can only spawn in non-peaceful difficulties
    NonPeaceful,
    /// Entity requires specific difficulty settings to spawn
    Custom(fn(u8) -> bool),
}

/// Defines the conditions under which an entity can spawn
pub struct SpawnRule {
    /// The entity kind this rule applies to
    pub entity_kind: EntityKind,
    /// Biomes where this entity can spawn
    pub biomes: Vec<BiomeId>,
    /// Minimum light level required for spawning
    pub min_light: u8,
    /// Maximum light level allowed for spawning
    pub max_light: u8,
    /// Minimum group size when spawned
    pub min_group_size: u32,
    /// Maximum group size when spawned
    pub max_group_size: u32,
    /// Specific block requirements for spawning
    pub required_blocks: Option<fn(BlockPosition) -> bool>,
    /// Difficulty requirements for spawning
    pub difficulty: SpawnDifficulty,
    /// Spawning weight (higher = more common)
    pub weight: u32,
    /// If true, entity spawns underwater
    pub aquatic: bool,
    /// If true, entity spawns in caves
    pub cave_spawn: bool,
    /// If true, entity requires sky access to spawn
    pub requires_sky_access: bool,
}

impl Default for SpawnRule {
    fn default() -> Self {
        Self {
            entity_kind: EntityKind::Zombie, // Default placeholder
            biomes: Vec::new(),
            min_light: 0,
            max_light: 15,
            min_group_size: 1,
            max_group_size: 4,
            required_blocks: None,
            difficulty: SpawnDifficulty::NonPeaceful,
            weight: 100,
            aquatic: false,
            cave_spawn: false,
            requires_sky_access: false,
        }
    }
}

/// Manages entity spawning rules
pub struct EntitySpawnManager {
    /// All registered spawn rules
    rules: Vec<SpawnRule>,
    /// Time until next spawn attempt
    next_spawn_time: Duration,
    /// Spawn rate controls how often entities spawn
    spawn_rate: Duration,
}

impl EntitySpawnManager {
    /// Creates a new entity spawn manager
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            next_spawn_time: Duration::from_secs(0),
            spawn_rate: Duration::from_secs(5),
        }
    }

    /// Registers a new spawn rule
    pub fn register_rule(&mut self, rule: SpawnRule) -> &mut Self {
        self.rules.push(rule);
        self
    }

    /// Attempts to spawn entities in the given chunk
    pub fn try_spawn_in_chunk<F, G>(
        &self,
        biome: BiomeId,
        chunk_pos: (i32, i32),
        light_getter: F,
        entity_spawner: G,
    ) 
    where
        F: Fn(BlockPosition) -> u8,
        G: FnMut(EntityKind, BlockPosition, u32),
    {
        let mut entity_spawner = entity_spawner;
        let mut rng = thread_rng();
        
        // Get all rules that apply to this biome
        let applicable_rules: Vec<&SpawnRule> = self.rules.iter()
            .filter(|rule| rule.biomes.contains(&biome))
            .collect();
        
        if applicable_rules.is_empty() {
            return;
        }
        
        // Choose a random rule based on weights
        let total_weight: u32 = applicable_rules.iter()
            .map(|rule| rule.weight)
            .sum();
        
        if total_weight == 0 {
            return;
        }
        
        let mut selection = rng.gen_range(0, total_weight);
        let selected_rule = applicable_rules.iter()
            .find(|rule| {
                if selection < rule.weight {
                    true
                } else {
                    selection -= rule.weight;
                    false
                }
            })
            .unwrap();
        
        // Choose random position in chunk
        let x = chunk_pos.0 * 16 + rng.gen_range(0, 16);
        let z = chunk_pos.1 * 16 + rng.gen_range(0, 16);
        
        // Find a valid Y position
        let y = self.find_spawn_y(x, z, selected_rule.aquatic);
        if y.is_none() {
            return;
        }
        let y = y.unwrap();
        
        let pos = BlockPosition::new(x, y, z);
        
        // Check light level
        let light = light_getter(pos);
        if light < selected_rule.min_light || light > selected_rule.max_light {
            return;
        }
        
        // Check custom block requirements
        if let Some(block_check) = selected_rule.required_blocks {
            if !block_check(pos) {
                return;
            }
        }
        
        // Determine group size
        let group_size = rng.gen_range(selected_rule.min_group_size, selected_rule.max_group_size + 1);
        
        // Spawn the entities
        entity_spawner(selected_rule.entity_kind, pos, group_size);
    }
    
    /// Finds a valid Y coordinate for spawning
    fn find_spawn_y(&self, x: i32, z: i32, aquatic: bool) -> Option<i32> {
        // This is a simplified version. In a real implementation, this would:
        // 1. For aquatic mobs, find water blocks
        // 2. For cave mobs, search caverns
        // 3. For surface mobs, find highest solid block
        // For now, we'll just return a placeholder y value
        if aquatic {
            Some(60) // Ocean level
        } else {
            Some(70) // Typical ground level
        }
    }
}

/// Register default spawn rules for all entities
pub fn register_default_spawn_rules(manager: &mut EntitySpawnManager) {
    // Register 1.17 entity spawn rules
    register_axolotl_rules(manager);
    register_goat_rules(manager);
    register_glow_squid_rules(manager);
    
    // Register other entity spawn rules
    register_animal_rules(manager);
    register_monster_rules(manager);
}

/// Register axolotl spawn rules
fn register_axolotl_rules(manager: &mut EntitySpawnManager) {
    manager.register_rule(SpawnRule {
        entity_kind: EntityKind::Axolotl,
        biomes: vec![BiomeId::LushCaves],
        min_light: 0,
        max_light: 15,
        min_group_size: 1,
        max_group_size: 4,
        required_blocks: Some(|pos| {
            // In real implementation, check if there's clay beneath
            true
        }),
        difficulty: SpawnDifficulty::Any,
        weight: 10,
        aquatic: true,
        cave_spawn: true,
        requires_sky_access: false,
    });
}

/// Register goat spawn rules
fn register_goat_rules(manager: &mut EntitySpawnManager) {
    manager.register_rule(SpawnRule {
        entity_kind: EntityKind::Goat,
        biomes: vec![BiomeId::FrozenPeaks, BiomeId::JaggedPeaks, BiomeId::SnowySlopes],
        min_light: 7,
        max_light: 15,
        min_group_size: 2,
        max_group_size: 3,
        required_blocks: Some(|pos| {
            // In real implementation, check for stone, snow, etc.
            true
        }),
        difficulty: SpawnDifficulty::Any,
        weight: 10,
        aquatic: false,
        cave_spawn: false,
        requires_sky_access: true,
    });
}

/// Register glow squid spawn rules
fn register_glow_squid_rules(manager: &mut EntitySpawnManager) {
    manager.register_rule(SpawnRule {
        entity_kind: EntityKind::GlowSquid,
        biomes: vec![BiomeId::Ocean, BiomeId::DeepOcean, BiomeId::LushCaves],
        min_light: 0,
        max_light: 0, // Only spawn in complete darkness
        min_group_size: 2,
        max_group_size: 4,
        required_blocks: None,
        difficulty: SpawnDifficulty::Any,
        weight: 10,
        aquatic: true,
        cave_spawn: true,
        requires_sky_access: false,
    });
}

/// Register animal spawn rules
fn register_animal_rules(manager: &mut EntitySpawnManager) {
    // Sheep
    manager.register_rule(SpawnRule {
        entity_kind: EntityKind::Sheep,
        biomes: vec![BiomeId::Plains, BiomeId::Forest, BiomeId::Taiga],
        min_light: 7,
        max_light: 15,
        min_group_size: 2,
        max_group_size: 4,
        required_blocks: None,
        difficulty: SpawnDifficulty::Any,
        weight: 12,
        aquatic: false,
        cave_spawn: false,
        requires_sky_access: true,
    });
    
    // Cows
    manager.register_rule(SpawnRule {
        entity_kind: EntityKind::Cow,
        biomes: vec![BiomeId::Plains, BiomeId::Forest],
        min_light: 7,
        max_light: 15,
        min_group_size: 2,
        max_group_size: 4,
        required_blocks: None,
        difficulty: SpawnDifficulty::Any,
        weight: 8,
        aquatic: false,
        cave_spawn: false,
        requires_sky_access: true,
    });
    
    // Pigs
    manager.register_rule(SpawnRule {
        entity_kind: EntityKind::Pig,
        biomes: vec![BiomeId::Plains, BiomeId::Forest],
        min_light: 7,
        max_light: 15,
        min_group_size: 2,
        max_group_size: 4,
        required_blocks: None,
        difficulty: SpawnDifficulty::Any,
        weight: 10,
        aquatic: false,
        cave_spawn: false,
        requires_sky_access: true,
    });
    
    // Additional animal rules would go here
}

/// Register monster spawn rules
fn register_monster_rules(manager: &mut EntitySpawnManager) {
    // Zombie
    manager.register_rule(SpawnRule {
        entity_kind: EntityKind::Zombie,
        biomes: vec![BiomeId::Plains, BiomeId::Forest, BiomeId::Desert, BiomeId::Taiga],
        min_light: 0,
        max_light: 0, // Only in complete darkness
        min_group_size: 2,
        max_group_size: 4,
        required_blocks: None,
        difficulty: SpawnDifficulty::NonPeaceful,
        weight: 100,
        aquatic: false,
        cave_spawn: true,
        requires_sky_access: false,
    });
    
    // Skeleton
    manager.register_rule(SpawnRule {
        entity_kind: EntityKind::Skeleton,
        biomes: vec![BiomeId::Plains, BiomeId::Forest, BiomeId::Desert, BiomeId::Taiga],
        min_light: 0,
        max_light: 0, // Only in complete darkness
        min_group_size: 1,
        max_group_size: 2,
        required_blocks: None,
        difficulty: SpawnDifficulty::NonPeaceful,
        weight: 80,
        aquatic: false,
        cave_spawn: true,
        requires_sky_access: false,
    });
    
    // Additional monster rules would go here
}

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(update_entity_spawning);
}

fn update_entity_spawning(game: &mut Game) -> SysResult {
    // This system would handle the timing of spawning attempts
    // and manage the spawning of entities in loaded chunks
    
    // In a real implementation, this would:
    // 1. Check if enough time has passed since last spawn attempt
    // 2. Select chunks for spawning attempts
    // 3. Call the entity spawn manager to try spawning in those chunks
    
    Ok(())
}