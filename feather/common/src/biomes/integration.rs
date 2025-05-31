use base::biome::Biome;
use base::chunk::BiomeStore;
use ecs::{Entity, IntoQuery, SysResult, SystemExecutor};
use quill_common::components::Position;
use std::collections::HashMap;

use crate::Game;
use crate::entities::spawning::EntitySpawnManager;

/// Handles integration of biomes with other game systems
pub struct BiomeIntegration {
    /// Maps entity types to the biomes they can spawn in
    entity_spawn_biomes: HashMap<EntityKind, Vec<Biome>>,
    /// Maps biomes to their ambient sound events
    biome_ambient_sounds: HashMap<Biome, SoundEvent>,
    /// Maps biomes to their climate characteristics
    biome_climate: HashMap<Biome, BiomeClimate>,
}

/// Climate characteristics of a biome
struct BiomeClimate {
    temperature: f32,
    humidity: f32,
    precipitation: bool,
}

impl BiomeIntegration {
    /// Creates a new biome integration system
    pub fn new() -> Self {
        let mut integration = Self {
            entity_spawn_biomes: HashMap::new(),
            biome_ambient_sounds: HashMap::new(),
            biome_climate: HashMap::new(),
        };
        
        integration.initialize_spawn_rules();
        integration.initialize_sound_mappings();
        integration.initialize_climate_data();
        
        integration
    }
    
    /// Initializes the entity spawning rules for each biome
    fn initialize_spawn_rules(&mut self) {
        // Axolotl spawn rules
        self.entity_spawn_biomes.insert(
            EntityKind::Axolotl,
            vec![Biome::LushCaves],
        );
        
        // Goat spawn rules
        self.entity_spawn_biomes.insert(
            EntityKind::Goat,
            vec![Biome::FrozenPeaks, Biome::JaggedPeaks, Biome::SnowySlopes, Biome::MountainGrove],
        );
        
        // Glow Squid spawn rules
        self.entity_spawn_biomes.insert(
            EntityKind::GlowSquid,
            vec![Biome::LushCaves, Biome::DeepDark, Biome::DeepOcean],
        );
        
        // Additional entity spawn rules would be added here
    }
    
    /// Initializes ambient sound mappings for biomes
    fn initialize_sound_mappings(&mut self) {
        // Implementation for biome sound mappings
    }
    
    /// Initializes climate data for biomes
    fn initialize_climate_data(&mut self) {
        // Implementation for biome climate characteristics
    }
    
    /// Gets the biome at a specific position in the world
    pub fn get_biome_at_position(&self, game: &Game, pos: Position) -> Option<Biome> {
        let block_pos = BlockPosition::from_position(pos);
        let chunk_pos = ChunkPosition::from_block_pos(block_pos);
        
        if let Some(chunk) = game.world.chunk_at(chunk_pos) {
            let biome_store = chunk.biomes();
            let relative_pos = block_pos.relative_to_chunk();
            
            // Convert to biome coordinates (biomes are stored at lower resolution)
            let biome_x = relative_pos.x as usize / BIOME_SAMPLE_RATE;
            let biome_y = relative_pos.y as usize / BIOME_SAMPLE_RATE;
            let biome_z = relative_pos.z as usize / BIOME_SAMPLE_RATE;
            
            return Some(biome_store.get(biome_x, biome_y, biome_z));
        }
        
        None
    }
    
    /// Determines if an entity can spawn at the given position based on biome
    pub fn can_entity_spawn_at(&self, game: &Game, entity_kind: EntityKind, pos: Position) -> bool {
        if let Some(biome) = self.get_biome_at_position(game, pos) {
            if let Some(allowed_biomes) = self.entity_spawn_biomes.get(&entity_kind) {
                return allowed_biomes.contains(&biome);
            }
        }
        
        false
    }
    
    /// Gets the ambient sound for a specific biome
    pub fn get_biome_ambient_sound(&self, biome: Biome) -> Option<SoundEvent> {
        self.biome_ambient_sounds.get(&biome).copied()
    }
    
    /// Gets the temperature of a biome adjusted for height
    pub fn get_adjusted_temperature(&self, biome: Biome, height: i32) -> f32 {
        if let Some(climate) = self.biome_climate.get(&biome) {
            let base_temp = climate.temperature;
            
            // Temperature decreases with height above Y=80
            if height > 80 {
                return base_temp - ((height - 80) as f32 * 0.00125);
            }
            
            return base_temp;
        }
        
        0.5 // Default moderate temperature
    }
    
    /// Updates entity states based on the biome they're in
    pub fn update_entity_states(&self, game: &mut Game) -> SysResult {
        for (entity, (position,)) in game.ecs.query::<(&Position,)>().iter() {
            if let Some(biome) = self.get_biome_at_position(game, *position) {
                // Apply biome-specific effects to entities
                self.apply_biome_effects(game, entity, biome, *position);
            }
        }
        
        Ok(())
    }
    
    /// Apply biome-specific effects to an entity
    fn apply_biome_effects(&self, game: &mut Game, entity: Entity, biome: Biome, pos: Position) {
        // Implementation for biome-specific entity effects
        match biome {
            Biome::Desert => {
                // Apply desert effects (e.g., drying, thirst, etc.)
            },
            Biome::SnowyTaiga | Biome::FrozenOcean | Biome::FrozenRiver => {
                // Apply cold biome effects (e.g., freezing, slowdown, etc.)
            },
            Biome::Swamp => {
                // Apply swamp effects (e.g., slowdown, poison resistance for certain mobs)
            },
            // Other biome-specific effects
            _ => {}
        }
    }
}

pub fn register(systems: &mut SystemExecutor<Game>) {
    let biome_integration = BiomeIntegration::new();
    systems.add_system_with_data(biome_integration.update_entity_states, biome_integration);
}