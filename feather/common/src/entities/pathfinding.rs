use base::{Position, BlockPosition, EntityKind};
use ecs::{Entity, IntoQuery, SysResult, SystemExecutor};
use quill_common::entities::{Axolotl, Goat, GlowSquid};
use quill_common::components::{OnGround, Velocity, Target, Path, PathNode, NavigationGoal};
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
use std::f32::consts::PI;

use crate::Game;

/// A node in the A* pathfinding algorithm
#[derive(Clone, Eq, PartialEq)]
struct AStarNode {
    position: BlockPosition,
    f_score: u32,  // Total cost (g_score + heuristic)
    g_score: u32,  // Cost from start
    parent: Option<BlockPosition>,
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems
        .add_system(update_axolotl_pathfinding)
        .add_system(update_goat_pathfinding)
        .add_system(update_glow_squid_pathfinding)
        .add_system(execute_paths);
}

/// Updates pathfinding for axolotls
fn update_axolotl_pathfinding(game: &mut Game) -> SysResult {
    for (entity, (axolotl, position, goal, path)) in game
        .ecs
        .query::<(&Axolotl, &Position, &NavigationGoal, &mut Path)>()
        .iter()
    {
        if path.needs_update {
            let start_pos = BlockPosition::from(*position);
            let target_pos = BlockPosition::from(goal.position);
            
            // Use specialized water-aware pathfinding for axolotls
            let new_path = find_water_aware_path(game, start_pos, target_pos);
            
            if let Some(nodes) = new_path {
                path.nodes = nodes;
                path.current_node = 0;
                path.needs_update = false;
            }
        }
    }
    
    Ok(())
}

/// Updates pathfinding for goats
fn update_goat_pathfinding(game: &mut Game) -> SysResult {
    for (entity, (goat, position, goal, path)) in game
        .ecs
        .query::<(&Goat, &Position, &NavigationGoal, &mut Path)>()
        .iter()
    {
        if path.needs_update {
            let start_pos = BlockPosition::from(*position);
            let target_pos = BlockPosition::from(goal.position);
            
            // Use specialized mountain-aware pathfinding for goats
            let new_path = find_mountain_aware_path(game, start_pos, target_pos);
            
            if let Some(nodes) = new_path {
                path.nodes = nodes;
                path.current_node = 0;
                path.needs_update = false;
            }
        }
    }
    
    Ok(())
}

/// Updates pathfinding for glow squids
fn update_glow_squid_pathfinding(game: &mut Game) -> SysResult {
    for (entity, (glow_squid, position, goal, path)) in game
        .ecs
        .query::<(&GlowSquid, &Position, &NavigationGoal, &mut Path)>()
        .iter()
    {
        if path.needs_update {
            let start_pos = BlockPosition::from(*position);
            let target_pos = BlockPosition::from(goal.position);
            
            // Use specialized underwater 3D pathfinding for glow squids
            let new_path = find_underwater_path(game, start_pos, target_pos);
            
            if let Some(nodes) = new_path {
                path.nodes = nodes;
                path.current_node = 0;
                path.needs_update = false;
            }
        }
    }
    
    Ok(())
}

/// Executes entity movement along calculated paths
fn execute_paths(game: &mut Game) -> SysResult {
    for (entity, (position, velocity, path)) in game
        .ecs
        .query::<(&mut Position, &mut Velocity, &mut Path)>()
        .iter()
    {
        if path.nodes.is_empty() || path.current_node >= path.nodes.len() {
            continue;
        }
        
        // Get current target node
        let target_node = &path.nodes[path.current_node];
        let target_pos = Position::new(
            target_node.position.x as f64 + 0.5,
            target_node.position.y as f64,
            target_node.position.z as f64 + 0.5
        );
        
        // Calculate vector to target
        let dx = target_pos.x - position.x;
        let dy = target_pos.y - position.y;
        let dz = target_pos.z - position.z;
        
        // Calculate distance to target node
        let dist_sq = dx * dx + dy * dy + dz * dz;
        
        if dist_sq < 0.5 {
            // Reached the node, move to next
            path.current_node += 1;
        } else {
            // Move towards target node
            let speed = 0.1; // Base movement speed
            let dist = dist_sq.sqrt();
            
            velocity.x = (dx / dist) * speed;
            velocity.z = (dz / dist) * speed;
            
            // Handle jumping or swimming upward if needed
            if dy > 0.5 && target_node.jump {
                velocity.y = 0.4; // Jump or swim upward
            }
        }
    }
    
    Ok(())
}

/// Finds a path using A* algorithm
fn find_path(game: &Game, start: BlockPosition, target: BlockPosition, max_iterations: usize) -> Option<Vec<PathNode>> {
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut g_scores = std::collections::HashMap::new();
    let mut came_from = std::collections::HashMap::new();
    
    // Add start node
    g_scores.insert(start, 0);
    open_set.push(AStarNode {
        position: start,
        f_score: manhattan_distance(start, target),
        g_score: 0,
        parent: None,
    });
    
    let mut iterations = 0;
    
    while let Some(current) = open_set.pop() {
        iterations += 1;
        
        if iterations > max_iterations {
            break; // Prevent excessive computation
        }
        
        // Check if we've reached the target
        if current.position == target {
            // Reconstruct path
            return Some(reconstruct_path(came_from, current.position));
        }
        
        // Add to closed set
        closed_set.insert(current.position);
        
        // Generate neighbors
        for neighbor in get_neighbors(game, current.position) {
            if closed_set.contains(&neighbor) {
                continue; // Skip already evaluated neighbors
            }
            
            // Calculate tentative g score
            let tentative_g = g_scores[&current.position] + 1;
            
            let mut add_to_open = false;
            
            // Check if this neighbor is new or if we found a better path
            if !g_scores.contains_key(&neighbor) || tentative_g < g_scores[&neighbor] {
                // Record the path
                came_from.insert(neighbor, current.position);
                g_scores.insert(neighbor, tentative_g);
                
                add_to_open = true;
            }
            
            if add_to_open {
                // Calculate f score (g + heuristic)
                let f_score = tentative_g + manhattan_distance(neighbor, target);
                
                open_set.push(AStarNode {
                    position: neighbor,
                    f_score,
                    g_score: tentative_g,
                    parent: Some(current.position),
                });
            }
        }
    }
    
    // No path found
    None
}

/// Specialized pathfinding for water movement (axolotls)
fn find_water_aware_path(game: &Game, start: BlockPosition, target: BlockPosition) -> Option<Vec<PathNode>> {
    // Custom implementation that prioritizes water blocks for axolotls
    // For now just use the base pathfinding algorithm
    find_path(game, start, target, 1000)
}

/// Specialized pathfinding for mountain movement (goats)
fn find_mountain_aware_path(game: &Game, start: BlockPosition, target: BlockPosition) -> Option<Vec<PathNode>> {
    // Custom implementation that allows for more vertical movement and jumps
    // For now just use the base pathfinding algorithm
    find_path(game, start, target, 1000)
}

/// Specialized pathfinding for underwater 3D movement (glow squids)
fn find_underwater_path(game: &Game, start: BlockPosition, target: BlockPosition) -> Option<Vec<PathNode>> {
    // Custom implementation for 3D underwater movement
    // For now just use the base pathfinding algorithm
    find_path(game, start, target, 1000)
}

/// Get valid neighboring positions
fn get_neighbors(game: &Game, pos: BlockPosition) -> Vec<BlockPosition> {
    // Basic neighbors (horizontally adjacent blocks)
    let basic_neighbors = vec![
        BlockPosition::new(pos.x + 1, pos.y, pos.z),
        BlockPosition::new(pos.x - 1, pos.y, pos.z),
        BlockPosition::new(pos.x, pos.y, pos.z + 1),
        BlockPosition::new(pos.x, pos.y, pos.z - 1),
    ];
    
    let mut valid_neighbors = Vec::new();
    
    for neighbor in basic_neighbors {
        // Check if the block is passable
        if is_passable_block(game, neighbor) {
            valid_neighbors.push(neighbor);
        }
    }
    
    // Check for vertical movement (up/down)
    let up = BlockPosition::new(pos.x, pos.y + 1, pos.z);
    let down = BlockPosition::new(pos.x, pos.y - 1, pos.z);
    
    // Can jump up one block
    if is_passable_block(game, up) && is_passable_block(game, BlockPosition::new(pos.x, pos.y + 2, pos.z)) {
        valid_neighbors.push(up);
    }
    
    // Can move down if the block below is solid or water
    if is_passable_block(game, down) {
        valid_neighbors.push(down);
    }
    
    valid_neighbors
}

/// Check if a block is passable
fn is_passable_block(game: &Game, pos: BlockPosition) -> bool {
    // This would check the block at the position to see if entities can pass through it
    // Implementation depends on how the world data is stored
    // For now, assume all blocks are passable
    true
}

/// Manhattan distance heuristic
fn manhattan_distance(a: BlockPosition, b: BlockPosition) -> u32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()) as u32
}

/// Reconstruct path from came_from map
fn reconstruct_path(came_from: std::collections::HashMap<BlockPosition, BlockPosition>, current: BlockPosition) -> Vec<PathNode> {
    let mut path = Vec::new();
    let mut current_pos = current;
    
    while let Some(&parent) = came_from.get(&current_pos) {
        // Check if this node requires jumping
        let jump = parent.y < current_pos.y;
        
        path.push(PathNode {
            position: current_pos,
            jump,
        });
        
        current_pos = parent;
    }
    
    // Add the start node
    path.push(PathNode {
        position: current_pos,
        jump: false,
    });
    
    // Reverse to get path from start to end
    path.reverse();
    path
}