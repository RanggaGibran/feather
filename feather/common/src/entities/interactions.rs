use base::{Position, EntityKind, Block, BlockPosition};
use ecs::{Entity, IntoQuery, SysResult, SystemExecutor};
use quill_common::entities::{Axolotl, PlayDead, Goat, RammingCooldown, GlowSquid, GlowIntensity};
use quill_common::components::{Health, OnGround, WaterBreathing, Velocity};
use crate::Game;

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems
        .add_system(update_axolotl_water_interactions)
        .add_system(update_goat_terrain_interactions)
        .add_system(update_glow_squid_water_interactions);
}

/// Handles axolotl interactions with water and land
fn update_axolotl_water_interactions(game: &mut Game) -> SysResult {
    for (entity, (axolotl, position, velocity, water_breathing)) in game
        .ecs
        .query::<(&Axolotl, &Position, &mut Velocity, &mut WaterBreathing)>()
        .iter()
    {
        // Get block the axolotl is in
        let block_pos = BlockPosition::from(*position);
        let in_water = is_block_water(game, block_pos);
        
        // Axolotls move faster in water, slower on land
        if in_water {
            // Replenish air supply in water
            water_breathing.air_ticks = water_breathing.max_air;
            
            // Boost swimming speed
            if velocity.magnitude() > 0.01 {
                velocity.x *= 1.2;
                velocity.z *= 1.2;
            }
        } else {
            // Reduce movement speed on land
            velocity.x *= 0.8;
            velocity.z *= 0.8;
            
            // Reduce air when out of water
            if water_breathing.air_ticks > 0 {
                water_breathing.air_ticks -= 5; // Lose air faster than normal mobs
            } else {
                // Apply damage when out of water too long
                if game.tick_count % 20 == 0 { // Once per second
                    if let Ok(mut health) = game.ecs.get_mut::<Health>(entity) {
                        health.current -= 1.0;
                    }
                }
            }
        }
        
        // Hunt nearby hostile water mobs (drowned, guardians)
        if game.tick_count % 10 == 0 { // Check every half second
            let nearby_hostiles = find_nearby_water_hostiles(game, *position, 8.0);
            for hostile in nearby_hostiles {
                // Target hostile mob for attack
                // This would connect with AI/targeting system
            }
        }
    }
    
    Ok(())
}

/// Handles goat interactions with terrain and other entities
fn update_goat_terrain_interactions(game: &mut Game) -> SysResult {
    for (entity, (goat, position, velocity, on_ground, ramming_cooldown)) in game
        .ecs
        .query::<(&Goat, &Position, &mut Velocity, &OnGround, &mut RammingCooldown)>()
        .iter()
    {
        // Check if standing on a mountable block
        let block_pos_below = BlockPosition::new(
            position.x as i32,
            position.y as i32 - 1,
            position.z as i32,
        );
        
        let is_mountain_block = is_block_mountain(game, block_pos_below);
        
        // Enhanced jumping on mountains
        if is_mountain_block && on_ground.0 && game.tick_count % 60 == 0 {
            if rand::random::<f32>() < 0.2 { // 20% chance to do a high jump
                // Apply upward velocity for jump
                velocity.y = 0.8; // Higher jump than normal
            }
        }
        
        // Handle ramming behavior
        if ramming_cooldown.ticks == 0 && on_ground.0 {
            // Find blocks that can drop items when rammed
            let front_block_pos = get_block_in_front(game, *position, 1.5);
            if can_ram_block(game, front_block_pos) {
                // Ram the block
                velocity.x *= 1.8;
                velocity.z *= 1.8;
                ramming_cooldown.ticks = 100;
                
                // Would trigger block break and potentially drop horn item
                trigger_block_ram(game, front_block_pos, entity);
            }
            
            // Find entities that can be rammed
            let entities_in_front = find_entities_in_front(game, *position, 3.0);
            if !entities_in_front.is_empty() {
                // Ram entity
                velocity.x *= 2.0;
                velocity.z *= 2.0;
                ramming_cooldown.ticks = 160;
                
                // Would cause knockback to entity
                apply_ram_knockback(game, entities_in_front[0], *position);
            }
        }
    }
    
    Ok(())
}

/// Handles glow squid interactions with water and light
fn update_glow_squid_water_interactions(game: &mut Game) -> SysResult {
    for (entity, (glow_squid, position, velocity, glow_intensity)) in game
        .ecs
        .query::<(&GlowSquid, &Position, &mut Velocity, &mut GlowIntensity)>()
        .iter()
    {
        let block_pos = BlockPosition::from(*position);
        let in_water = is_block_water(game, block_pos);
        
        if !in_water {
            // Glow squids can only survive in water
            if game.tick_count % 10 == 0 { // Every half second
                if let Ok(mut health) = game.ecs.get_mut::<Health>(entity) {
                    health.current -= 2.0;
                }
            }
            
            // Reduced movement on land
            velocity.x *= 0.2;
            velocity.z *= 0.2;
        } else {
            // Smooth swimming motion in water
            if velocity.magnitude() > 0.01 {
                velocity.y *= 0.98; // Slows vertical movement for smoother swimming
            }
        }
        
        // Check light level and adjust glow intensity
        let light_level = get_light_level(game, block_pos);
        if light_level > 8 { // In bright areas
            glow_intensity.value *= 0.95; // Glow dims in brighter areas
            if glow_intensity.value < 0.4 {
                glow_intensity.value = 0.4;
            }
        } else {
            // In dark areas, glow is stronger
            glow_intensity.value = (glow_intensity.value * 1.05).min(1.0);
        }
        
        // Ink particles when attacked
        if let Ok(health) = game.ecs.get::<Health>(entity) {
            if health.current < health.max * 0.7 && game.tick_count % 40 == 0 {
                spawn_glow_ink_particles(game, *position);
            }
        }
    }
    
    Ok(())
}

// Helper functions
fn is_block_water(game: &Game, block_pos: BlockPosition) -> bool {
    // This would check the block type at the position
    // Implementation depends on how block data is stored
    // Simplified version for demonstration
    false
}

fn is_block_mountain(game: &Game, block_pos: BlockPosition) -> bool {
    // Would check if block is stone, andesite, or other mountain blocks
    // Simplified version for demonstration
    false
}

fn get_block_in_front(game: &Game, position: Position, distance: f32) -> BlockPosition {
    // Calculate position in front based on entity rotation
    // Simplified version for demonstration
    BlockPosition::new(
        position.x as i32 + 1,
        position.y as i32,
        position.z as i32,
    )
}

fn can_ram_block(game: &Game, block_pos: BlockPosition) -> bool {
    // Check if block is rammable (would drop horn, etc)
    // Simplified version for demonstration
    false
}

fn trigger_block_ram(game: &Game, block_pos: BlockPosition, entity: Entity) {
    // Would trigger block breaking or item drops
    // Simplified version for demonstration
}

fn find_entities_in_front(game: &Game, position: Position, distance: f32) -> Vec<Entity> {
    // Find entities in front of the position
    // Simplified version for demonstration
    Vec::new()
}

fn apply_ram_knockback(game: &Game, target: Entity, source_pos: Position) {
    // Apply knockback to target entity
    // Simplified version for demonstration
}

fn find_nearby_water_hostiles(game: &Game, position: Position, radius: f32) -> Vec<Entity> {
    // Find hostile water mobs near the position
    // Simplified version for demonstration
    Vec::new()
}

fn get_light_level(game: &Game, block_pos: BlockPosition) -> u8 {
    // Get light level at block position
    // Simplified version for demonstration
    0
}

fn spawn_glow_ink_particles(game: &Game, position: Position) {
    // Spawn glow ink particle effects
    // Simplified version for demonstration
}