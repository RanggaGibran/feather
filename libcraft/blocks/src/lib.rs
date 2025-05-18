mod block;
mod block_data;
pub mod data;
mod registry;
mod simplified_block;
mod block_properties;
mod behaviors;
mod registration;
mod block_transitions;
mod block_ticking;
mod tick_executor;
mod chunk_integration;
mod block_entity;

pub use block::BlockKind;
pub use block_data::*;
pub use registry::BlockState;
pub use simplified_block::SimplifiedBlockKind;
pub use block_properties::{BlockProperties, BlockBehavior, DefaultBlockBehavior};
pub use behaviors::{DoorBehavior, ChestBehavior, RedstoneBehavior, get_behavior_for_block};
pub use registration::BlockRegistry;
pub use block_transitions::{BlockTransitionManager, BlockStateTransition, TransitionCondition};
pub use block_ticking::{BlockTickScheduler, BlockTick, TickType};
pub use tick_executor::BlockTickExecutor;
pub use chunk_integration::BlockWorldIntegration;
pub use block_entity::{BlockEntity, BlockEntityKind, BlockEntityData, BlockEntityManager, BlockEntityValue, 
                      create_block_entity, requires_block_entity, serialize_block_entity, deserialize_block_entity};

// Add a convenience method to BlockKind
impl BlockKind {
    pub fn get_behavior(&self) -> Box<dyn BlockBehavior> {
        behaviors::get_behavior_for_block(*self)
    }
    
    pub fn default_properties(&self) -> BlockProperties {
        let mut props = BlockProperties::new(*self);
        
        // Set default properties based on block kind
        match self {
            BlockKind::OakDoor | BlockKind::SpruceDoor | BlockKind::BirchDoor |
            BlockKind::JungleDoor | BlockKind::AcaciaDoor | BlockKind::DarkOakDoor |
            BlockKind::CrimsonDoor | BlockKind::WarpedDoor | BlockKind::IronDoor => {
                props.set("half", "lower")
                    .set("facing", "north")
                    .set("open", "false")
                    .set("hinge", "left")
                    .set("powered", "false");
            },
            
            BlockKind::Chest | BlockKind::TrappedChest => {
                props.set("facing", "north")
                    .set("waterlogged", "false")
                    .set("type", "single");
            },
            
            BlockKind::Lantern | BlockKind::SoulLantern => {
                props.set("hanging", "false")
                    .set("waterlogged", "false");
            },
            
            // Add 1.17 blocks properties
            BlockKind::Copper | BlockKind::ExposedCopper | 
            BlockKind::WeatheredCopper | BlockKind::OxidizedCopper => {
                // Copper blocks have no properties in their basic form
            },
            
            BlockKind::CutCopper | BlockKind::ExposedCutCopper | 
            BlockKind::WeatheredCutCopper | BlockKind::OxidizedCutCopper => {
                // Cut copper blocks have no properties in their basic form
            },
            
            BlockKind::CutCopperStairs | BlockKind::ExposedCutCopperStairs | 
            BlockKind::WeatheredCutCopperStairs | BlockKind::OxidizedCutCopperStairs => {
                props.set("facing", "north")
                    .set("half", "bottom")
                    .set("shape", "straight")
                    .set("waterlogged", "false");
            },
            
            BlockKind::CutCopperSlab | BlockKind::ExposedCutCopperSlab | 
            BlockKind::WeatheredCutCopperSlab | BlockKind::OxidizedCutCopperSlab => {
                props.set("type", "bottom")
                    .set("waterlogged", "false");
            },
            
            BlockKind::LightningRod => {
                props.set("facing", "up")
                    .set("powered", "false")
                    .set("waterlogged", "false");
            },
            
            BlockKind::Candle | BlockKind::WhiteCandle | BlockKind::OrangeCandle |
            BlockKind::MagentaCandle | BlockKind::LightBlueCandle | BlockKind::YellowCandle |
            BlockKind::LimeCandle | BlockKind::PinkCandle | BlockKind::GrayCandle |
            BlockKind::LightGrayCandle | BlockKind::CyanCandle | BlockKind::PurpleCandle |
            BlockKind::BlueCandle | BlockKind::BrownCandle | BlockKind::GreenCandle |
            BlockKind::RedCandle | BlockKind::BlackCandle => {
                props.set("candles", "1")
                    .set("lit", "false")
                    .set("waterlogged", "false");
            },
            
            BlockKind::PointedDripstone => {
                props.set("thickness", "tip")
                    .set("vertical_direction", "up")
                    .set("waterlogged", "false");
            },
            
            BlockKind::AmethystCluster => {
                props.set("facing", "up")
                    .set("waterlogged", "false");
            },
            
            // Add more default properties for other block types
            _ => {}
        }
        
        props
    }
    
    /// Check if this block can receive random ticks
    pub fn receives_random_ticks(&self) -> bool {
        match self {
            // List blocks that should receive random ticks
            BlockKind::Copper | BlockKind::ExposedCopper | BlockKind::WeatheredCopper => true,
            BlockKind::CutCopper | BlockKind::ExposedCutCopper | BlockKind::WeatheredCutCopper => true,
            BlockKind::CutCopperStairs | BlockKind::ExposedCutCopperStairs | BlockKind::WeatheredCutCopperStairs => true,
            BlockKind::CutCopperSlab | BlockKind::ExposedCutCopperSlab | BlockKind::WeatheredCutCopperSlab => true,
            BlockKind::BuddingAmethyst => true,
            BlockKind::PointedDripstone => true,
            _ => false,
        }
    }
}

pub fn initialize_block_registry() -> BlockRegistry {
    let mut registry = BlockRegistry::new();
    
    // Register vanilla blocks
    registry.register_block("stone", BlockKind::Stone);
    registry.register_block("grass_block", BlockKind::GrassBlock);
    registry.register_block("dirt", BlockKind::Dirt);
    
    // Register 1.17 blocks
    registry.register_block("copper_block", BlockKind::Copper);
    registry.register_block("exposed_copper", BlockKind::ExposedCopper);
    registry.register_block("weathered_copper", BlockKind::WeatheredCopper);
    registry.register_block("oxidized_copper", BlockKind::OxidizedCopper);
    
    registry.register_block("cut_copper", BlockKind::CutCopper);
    registry.register_block("exposed_cut_copper", BlockKind::ExposedCutCopper);
    registry.register_block("weathered_cut_copper", BlockKind::WeatheredCutCopper);
    registry.register_block("oxidized_cut_copper", BlockKind::OxidizedCutCopper);
    
    registry.register_block("amethyst_block", BlockKind::AmethystBlock);
    registry.register_block("budding_amethyst", BlockKind::BuddingAmethyst);
    registry.register_block("amethyst_cluster", BlockKind::AmethystCluster);
    
    registry.register_block("tinted_glass", BlockKind::TintedGlass);
    
    registry.register_block("lightning_rod", BlockKind::LightningRod)
        .register_behavior(BlockKind::LightningRod, behaviors::RedstoneBehavior);
    
    registry.register_block("candle", BlockKind::Candle);
    
    // Register behaviors for existing blocks
    registry.register_behavior(BlockKind::OakDoor, behaviors::DoorBehavior);
    registry.register_behavior(BlockKind::Chest, behaviors::ChestBehavior);
    registry.register_behavior(BlockKind::RedstoneWire, behaviors::RedstoneBehavior);
    
    registry
}

/// Initializes a new BlockTransitionManager with default transitions
pub fn initialize_block_transitions() -> BlockTransitionManager {
    let mut manager = BlockTransitionManager::new();
    
    // Register copper transitions
    block_transitions::register_copper_transitions(&mut manager);
    
    // Add more transition registrations here
    
    manager
}

/// Initialize a new BlockTickExecutor with default settings
pub fn initialize_block_tick_executor() -> BlockTickExecutor {
    // Default random tick speed (3 is Minecraft's default)
    let random_tick_speed = 3;
    let transition_manager = initialize_block_transitions();
    
    BlockTickExecutor::new(random_tick_speed, transition_manager)
}

// Helper function to create a BlockWorldIntegration
pub fn initialize_block_world_integration() -> BlockWorldIntegration {
    let tick_executor = initialize_block_tick_executor();
    BlockWorldIntegration::new(tick_executor)
}
