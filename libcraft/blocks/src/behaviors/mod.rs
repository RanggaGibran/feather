mod door;
mod chest;
mod redstone;

pub use door::DoorBehavior;
pub use chest::ChestBehavior;
pub use redstone::RedstoneBehavior;

pub fn get_behavior_for_block(kind: crate::BlockKind) -> Box<dyn crate::BlockBehavior> {
    match kind {
        crate::BlockKind::OakDoor | 
        crate::BlockKind::SpruceDoor |
        crate::BlockKind::BirchDoor |
        crate::BlockKind::JungleDoor |
        crate::BlockKind::AcaciaDoor |
        crate::BlockKind::DarkOakDoor |
        crate::BlockKind::IronDoor => Box::new(door::DoorBehavior),
        
        crate::BlockKind::Chest |
        crate::BlockKind::TrappedChest => Box::new(chest::ChestBehavior),
        
        crate::BlockKind::RedstoneWire |
        crate::BlockKind::RedstoneTorch |
        crate::BlockKind::RedstoneBlock => Box::new(redstone::RedstoneBehavior),
        
        _ => Box::new(crate::DefaultBlockBehavior),
    }
}