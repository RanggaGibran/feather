use std::collections::HashMap;
use crate::{BlockKind, BlockProperties, BlockBehavior, DefaultBlockBehavior};
use ahash::AHashMap;

/// Manages registration of blocks and their properties
pub struct BlockRegistry {
    registered_blocks: AHashMap<String, BlockRegistration>,
    behavior_registry: AHashMap<BlockKind, Box<dyn BlockBehavior>>,
}

/// Contains registration information for a specific block
pub struct BlockRegistration {
    pub kind: BlockKind,
    pub properties: BlockProperties,
    pub default_state: u16,
    pub states: Vec<u16>,
}

impl BlockRegistry {
    /// Creates a new, empty block registry
    pub fn new() -> Self {
        Self {
            registered_blocks: AHashMap::new(),
            behavior_registry: AHashMap::new(),
        }
    }

    /// Registers a block with its default properties
    pub fn register_block(&mut self, name: &str, kind: BlockKind) -> &mut Self {
        let properties = kind.default_properties();
        let registration = BlockRegistration {
            kind,
            properties,
            default_state: 0, // Will be populated later
            states: Vec::new(),
        };
        
        self.registered_blocks.insert(name.to_string(), registration);
        self
    }

    /// Registers a custom behavior for a block
    pub fn register_behavior<B: BlockBehavior + 'static>(&mut self, kind: BlockKind, behavior: B) -> &mut Self {
        self.behavior_registry.insert(kind, Box::new(behavior));
        self
    }

    /// Gets the behavior for a block kind
    pub fn get_behavior(&self, kind: BlockKind) -> Box<dyn BlockBehavior> {
        match self.behavior_registry.get(&kind) {
            Some(behavior) => behavior.clone(),
            None => Box::new(DefaultBlockBehavior)
        }
    }

    /// Loads block data from a configuration file
    pub fn load_from_config(&mut self, config_path: &str) -> Result<(), std::io::Error> {
        // Implementation would load data from a JSON or TOML config file
        // For now just a placeholder
        Ok(())
    }
}

// Trait to allow cloning of boxed BlockBehavior
trait CloneBlockBehavior {
    fn clone(&self) -> Box<dyn BlockBehavior>;
}

// Implement clone for any type that implements BlockBehavior
impl<T: BlockBehavior + 'static> CloneBlockBehavior for T {
    fn clone(&self) -> Box<dyn BlockBehavior> {
        Box::new(self.clone())
    }
}

// Implement Clone for boxed BlockBehavior
impl Clone for Box<dyn BlockBehavior> {
    fn clone(&self) -> Self {
        self.as_ref().clone()
    }
}