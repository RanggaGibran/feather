use crate::{BlockBehavior, BlockProperties};

pub struct DoorBehavior;

impl BlockBehavior for DoorBehavior {
    fn on_placed(&self, _properties: &BlockProperties) {
        // Implementation for door placement
    }
    
    fn on_broken(&self, _properties: &BlockProperties) {
        // When breaking a door, we may need to break the other half as well
    }
    
    fn can_interact(&self, properties: &BlockProperties) -> bool {
        // Iron doors cannot be interacted with directly
        properties.kind() != crate::BlockKind::IronDoor
    }
    
    fn on_interact(&self, properties: &BlockProperties) -> bool {
        if !self.can_interact(properties) {
            return false;
        }
        
        // Toggle the door state
        // In a real implementation, we would modify the block state
        true
    }
}