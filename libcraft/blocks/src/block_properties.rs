use std::collections::HashMap;
use crate::BlockKind;

/// Represents the properties a block can have
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockProperties {
    /// Map of property name to property value
    properties: HashMap<String, String>,
    /// The kind of block these properties belong to
    kind: BlockKind,
}

impl BlockProperties {
    /// Creates a new BlockProperties with the given kind
    pub fn new(kind: BlockKind) -> Self {
        Self {
            properties: HashMap::new(),
            kind,
        }
    }

    /// Sets a property value
    pub fn set(&mut self, name: &str, value: &str) -> &mut Self {
        self.properties.insert(name.to_string(), value.to_string());
        self
    }

    /// Gets a property value
    pub fn get(&self, name: &str) -> Option<&String> {
        self.properties.get(name)
    }

    /// Returns the kind of block
    pub fn kind(&self) -> BlockKind {
        self.kind
    }

    /// Returns all properties
    pub fn all(&self) -> &HashMap<String, String> {
        &self.properties
    }
}

/// Defines the behavior of a block
pub trait BlockBehavior {
    /// Called when a block is placed
    fn on_placed(&self, properties: &BlockProperties);
    
    /// Called when a block is broken
    fn on_broken(&self, properties: &BlockProperties);
    
    /// Whether this block can be interacted with
    fn can_interact(&self, properties: &BlockProperties) -> bool;
    
    /// Called when a block is interacted with
    fn on_interact(&self, properties: &BlockProperties) -> bool;
}

/// Default implementation of BlockBehavior that does nothing
pub struct DefaultBlockBehavior;

impl BlockBehavior for DefaultBlockBehavior {
    fn on_placed(&self, _properties: &BlockProperties) {}
    fn on_broken(&self, _properties: &BlockProperties) {}
    fn can_interact(&self, _properties: &BlockProperties) -> bool { false }
    fn on_interact(&self, _properties: &BlockProperties) -> bool { false }
}