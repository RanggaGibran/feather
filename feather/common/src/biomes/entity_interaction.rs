use base::biome::Biome;
use base::Position;
use base::EntityKind;
use crate::Game;
use ecs::{Entity, IntoQuery, SysResult, SystemExecutor};
use quill_common::components::{Health, MovementSpeed, Damage, StatusEffect, StatusEffectKind};
use std::time::Duration;

/// Handles specific interactions between biomes and entities
pub struct BiomeEntityInteraction {
    /// How often to check for biome effects (in ticks)
    update_interval: u32,
    /// Current tick counter
    tick_counter: u32,
}

impl BiomeEntityInteraction {
    pub fn new() -> Self {
        Self {
            update_interval: 20, // Check every second (20 ticks)
            tick_counter: 0,
        }
    }

    /// Updates entity states based on their current biome
    pub fn update(&mut self, game: &mut Game) -> SysResult {
        self.tick_counter += 1;
        
        // Only process on the specified interval
        if self.tick_counter % self.update_interval != 0 {
            return Ok(());
        }
        
        // Get all entities with positions
        for (entity, (position, entity_kind)) in game.ecs.query::<(&Position, &EntityKind)>().iter() {
            // Get the biome at entity's position
            if let Some(biome) = game.biome_integration.get_biome_at_position(*position) {
                // Apply biome-specific effects based on entity type
                match entity_kind {
                    EntityKind::Axolotl => self.apply_axolotl_biome_effects(game, entity, biome),
                    EntityKind::Goat => self.apply_goat_biome_effects(game, entity, biome),
                    EntityKind::GlowSquid => self.apply_glow_squid_biome_effects(game, entity, biome),
                    _ => self.apply_general_biome_effects(game, entity, *entity_kind, biome),
                }
            }
        }
        
        Ok(())
    }
    
    /// Apply biome effects specific to Axolotls
    fn apply_axolotl_biome_effects(&self, game: &mut Game, entity: Entity, biome: Biome) {
        match biome {
            Biome::LushCaves => {
                // Axolotls thrive in lush caves - regenerate health
                if let Ok(mut health) = game.ecs.get_mut::<Health>(entity) {
                    health.current += 0.5;
                    if health.current > health.max {
                        health.current = health.max;
                    }
                }
            },
            Biome::Desert | Biome::Badlands | Biome::SavannaPlateau => {
                // Axolotls suffer in dry biomes - take damage over time
                if let Ok(mut health) = game.ecs.get_mut::<Health>(entity) {
                    health.current -= 1.0;
                }
            },
            _ => {
                // No special effects in other biomes
            }
        }
    }
    
    /// Apply biome effects specific to Goats
    fn apply_goat_biome_effects(&self, game: &mut Game, entity: Entity, biome: Biome) {
        match biome {
            Biome::FrozenPeaks | Biome::JaggedPeaks | Biome::SnowySlopes => {
                // Goats are faster in mountain biomes
                if let Ok(mut speed) = game.ecs.get_mut::<MovementSpeed>(entity) {
                    speed.value = speed.base_value * 1.2;
                }
            },
            Biome::Swamp | Biome::MangroveSwamp => {
                // Goats are slower in swamp biomes
                if let Ok(mut speed) = game.ecs.get_mut::<MovementSpeed>(entity) {
                    speed.value = speed.base_value * 0.8;
                }
            },
            _ => {
                // Reset to normal speed in other biomes
                if let Ok(mut speed) = game.ecs.get_mut::<MovementSpeed>(entity) {
                    speed.value = speed.base_value;
                }
            }
        }
    }
    
    /// Apply biome effects specific to Glow Squids
    fn apply_glow_squid_biome_effects(&self, game: &mut Game, entity: Entity, biome: Biome) {
        // Get the light level at the entity's position
        let position = game.ecs.get::<Position>(entity).unwrap();
        let light_level = game.world.get_light_level_at(*position);
        
        match biome {
            Biome::DeepDark | Biome::LushCaves => {
                // Increase glow intensity in dark biomes
                if let Ok(mut glow) = game.ecs.get_mut::<GlowIntensity>(entity) {
                    glow.value = (glow.value * 1.05).min(1.0);
                }
            },
            _ => {
                // Decrease glow intensity in bright areas or other biomes
                if light_level > 8 {
                    if let Ok(mut glow) = game.ecs.get_mut::<GlowIntensity>(entity) {
                        glow.value = (glow.value * 0.95).max(0.4);
                    }
                }
            }
        }
    }
    
    /// Apply general biome effects to all entity types
    fn apply_general_biome_effects(&self, game: &mut Game, entity: Entity, entity_kind: EntityKind, biome: Biome) {
        // Apply effects based on entity category (hostile, passive, etc.)
        let is_undead = matches!(
            entity_kind,
            EntityKind::Zombie | EntityKind::Skeleton | EntityKind::ZombieVillager | 
            EntityKind::WitherSkeleton | EntityKind::Stray | EntityKind::Drowned | 
            EntityKind::Phantom | EntityKind::ZombifiedPiglin
        );
        
        if is_undead {
            match biome {
                Biome::Desert => {
                    // Undead mobs don't burn in daylight in desert
                    if let Ok(mut status) = game.ecs.get_mut::<StatusEffect>(entity) {
                        status.remove(StatusEffectKind::BurningInDaylight);
                    }
                },
                _ => {}
            }
        }
        
        // Apply temperature effects
        let position = game.ecs.get::<Position>(entity).unwrap();
        let temperature = game.biome_integration.get_adjusted_temperature(biome, position.y as i32);
        
        if temperature < 0.1 {
            // Very cold biomes slow down non-cold entities
            if !matches!(entity_kind, EntityKind::SnowGolem | EntityKind::Stray | EntityKind::PolarBear) {
                if let Ok(mut speed) = game.ecs.get_mut::<MovementSpeed>(entity) {
                    speed.value = speed.base_value * 0.9;
                }
            }
        } else if temperature > 1.5 {
            // Very hot biomes damage cold entities
            if matches!(entity_kind, EntityKind::SnowGolem) {
                if let Ok(mut health) = game.ecs.get_mut::<Health>(entity) {
                    health.current -= 0.5;
                }
            }
        }
    }
}

pub fn register(systems: &mut SystemExecutor<Game>) {
    let interaction = BiomeEntityInteraction::new();
    systems.add_system_with_data(interaction.update, interaction);
}