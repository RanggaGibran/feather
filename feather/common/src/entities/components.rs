use base::{Position, BlockPosition};
use ecs::Component;

/// A path followed by an entity
#[derive(Component, Default, Debug, Clone)]
pub struct Path {
    pub nodes: Vec<PathNode>,
    pub current_node: usize,
    pub needs_update: bool,
}

/// A node in a path
#[derive(Debug, Clone)]
pub struct PathNode {
    pub position: BlockPosition,
    pub jump: bool,
}

/// A navigation goal for an entity
#[derive(Component, Debug, Clone)]
pub struct NavigationGoal {
    pub position: Position,
    pub tolerance: f64,
    pub priority: u8,
}

/// A target for an entity to follow or attack
#[derive(Component, Debug)]
pub struct Target {
    pub entity: Option<Entity>,
    pub position: Option<Position>,
    pub timer: u32,
    pub is_hostile: bool,
}