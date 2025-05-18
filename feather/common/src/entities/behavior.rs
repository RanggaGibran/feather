use base::{Position, EntityKind};
use ecs::{Entity, IntoQuery, SysResult, SystemExecutor};
use quill_common::entities::{Axolotl, PlayDead, Goat, RammingCooldown, GlowSquid, GlowIntensity};
use quill_common::components::{Damage, Health, OnGround, WaterBreathing};

use crate::Game;

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems
        .add_system(update_axolotl_behavior)
        .add_system(update_goat_behavior)
        .add_system(update_glow_squid_behavior);
}

fn update_axolotl_behavior(game: &mut Game) -> SysResult {
    // Handle play dead behavior
    for (entity, (axolotl, play_dead, health)) in game.ecs.query::<(&Axolotl, &mut PlayDead, &Health)>().iter() {
        // Check if axolotl should play dead (below 30% health)
        if health.current < health.max * 0.3 && !play_dead.activated {
            if rand::random::<f32>() < 0.30 { // 30% chance to play dead when hit and low health
                play_dead.activated = true;
                play_dead.timer = 120; // 6 seconds (20 ticks per second)
            }
        }
        
        // Update play dead timer
        if play_dead.activated && play_dead.timer > 0 {
            play_dead.timer -= 1;
            
            if play_dead.timer == 0 {
                play_dead.activated = false;
                // Heal axolotl when it recovers from playing dead
                if let Ok(mut health) = game.ecs.get_mut::<Health>(entity) {
                    health.current += health.max * 0.2; // Heal 20% of max health
                    health.current = health.current.min(health.max); // Cap at max health
                }
            }
        }
    }
    
    // Handle water breathing
    for (_, (axolotl, water_breathing, position)) in game
        .ecs
        .query::<(&Axolotl, &mut WaterBreathing, &Position)>()
        .iter()
    {
        // Check if axolotl is in water
        let in_water = is_in_water(game, *position);
        
        if in_water {
            // Axolotls regain air quickly in water
            water_breathing.air_ticks = water_breathing.max_air;
        } else {
            // Axolotls lose air when not in water
            if water_breathing.air_ticks > 0 {
                water_breathing.air_ticks -= 1;
            } else {
                // Take damage when out of air
                // This would be implemented in a separate damage system
            }
        }
    }
    
    Ok(())
}

fn update_goat_behavior(game: &mut Game) -> SysResult {
    // Handle ramming cooldown
    for (_, (goat, ramming_cooldown)) in game.ecs.query::<(&Goat, &mut RammingCooldown)>().iter() {
        if ramming_cooldown.ticks > 0 {
            ramming_cooldown.ticks -= 1;
        }
    }
    
    // Handle goat ramming behavior - simplified for this example
    for (entity, (goat, ramming_cooldown, position, on_ground)) in game
        .ecs
        .query::<(&Goat, &mut RammingCooldown, &Position, &OnGround)>()
        .iter()
    {
        if ramming_cooldown.ticks == 0 && on_ground.0 {
            // Look for nearby entities to ram
            // This is a simplified version - in reality, you would check for entities in view
            let potential_targets = find_nearby_entities(game, *position, 8.0);
            
            for target in potential_targets {
                if rand::random::<f32>() < 0.1 { // 10% chance to ram when a target is found
                    // Set ramming cooldown
                    ramming_cooldown.ticks = 200 + (rand::random::<u32>() % 200); // 10-20 seconds
                    
                    // Apply knockback to target
                    // This would be implemented in a separate physics system
                    
                    break;
                }
            }
        }
    }
    
    Ok(())
}

fn update_glow_squid_behavior(game: &mut Game) -> SysResult {
    // Handle glow intensity pulsing
    for (_, (glow_squid, glow_intensity, health)) in game
        .ecs
        .query::<(&GlowSquid, &mut GlowIntensity, &Health)>()
        .iter()
    {
        // Make glow intensity pulse over time
        let time = game.tick_count % 60; // 3-second pulse cycle
        let pulse_factor = (time as f32 / 30.0 * std::f32::consts::PI).sin() * 0.2 + 0.8;
        
        // Reduce glow when damaged
        let health_factor = health.current / health.max;
        
        glow_intensity.value = pulse_factor * health_factor;
    }
    
    Ok(())
}

// Helper function to check if a position is in water
fn is_in_water(game: &Game, position: Position) -> bool {
    // This would check the block at the position and return true if it's water
    // Simplified for this implementation
    false
}

// Helper function to find nearby entities
fn find_nearby_entities(game: &Game, position: Position, radius: f32) -> Vec<Entity> {
    // This would return entities within radius of the position
    // Simplified for this implementation
    Vec::new()
}