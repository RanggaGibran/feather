use std::time::Duration;
use crate::{BlockKind, BlockProperties, BlockTransitionManager};
use crate::block_ticking::{BlockTickScheduler, TickType};

/// Executes ticks for blocks
pub struct BlockTickExecutor {
    scheduler: BlockTickScheduler,
    transition_manager: BlockTransitionManager,
}

impl BlockTickExecutor {
    /// Creates a new block tick executor
    pub fn new(random_tick_speed: u32, transition_manager: BlockTransitionManager) -> Self {
        Self {
            scheduler: BlockTickScheduler::new(random_tick_speed),
            transition_manager,
        }
    }
    
    /// Schedules a tick for a block
    pub fn schedule_tick(
        &mut self,
        position: (i32, i32, i32),
        kind: BlockKind,
        delay: Duration,
        priority: i32,
    ) {
        self.scheduler.schedule_tick(
            position,
            kind,
            delay,
            TickType::Scheduled,
            priority,
        );
    }
    
    /// Process ticks that are due
    pub fn process_ticks<F, G>(
        &mut self,
        block_getter: F,
        block_setter: G,
    )
    where
        F: Fn((i32, i32, i32)) -> Option<(BlockKind, BlockProperties)>,
        G: FnMut((i32, i32, i32), BlockKind),
    {
        let mut block_setter = block_setter;
        let transition_manager = &self.transition_manager;
        
        self.scheduler.process_ticks(move |pos, kind, tick_type| {
            if let Some((current_kind, properties)) = block_getter(pos) {
                // Verify the block is still the same type
                if current_kind != kind {
                    return;
                }
                
                // Check for transitions
                if let Some(target_kind) = transition_manager.check_transition(current_kind, &properties) {
                    block_setter(pos, target_kind);
                }
                
                // Execute the tick behavior based on the block type
                match current_kind {
                    BlockKind::Copper | 
                    BlockKind::ExposedCopper |
                    BlockKind::WeatheredCopper => {
                        // Copper weathering will be handled by transitions
                    },
                    BlockKind::BuddingAmethyst => {
                        // Chance to grow amethyst buds on adjacent blocks
                        if tick_type == TickType::Random {
                            // Implementation for amethyst growth
                        }
                    },
                    BlockKind::PointedDripstone => {
                        // Handle dripstone growth
                    },
                    // Handle other blocks with tick behavior
                    _ => {}
                }
            }
        });
    }
    
    /// Process random ticks for a chunk section
    pub fn process_random_ticks<F, G>(
        &self,
        chunk_position: (i32, i32),
        blocks: &[(BlockKind, (i32, i32, i32), BlockProperties)],
        block_getter: F,
        mut block_setter: G,
    )
    where
        F: Fn((i32, i32, i32)) -> Option<(BlockKind, BlockProperties)>,
        G: FnMut((i32, i32, i32), BlockKind),
    {
        let transition_manager = &self.transition_manager;
        
        self.scheduler.process_random_ticks(chunk_position, blocks, move |pos, kind| {
            if let Some((current_kind, properties)) = block_getter(pos) {
                // Execute random tick behavior
                if let Some(target_kind) = transition_manager.check_transition(current_kind, &properties) {
                    block_setter(pos, target_kind);
                }
            }
        });
    }
    
    /// Get a reference to the scheduler
    pub fn scheduler(&self) -> &BlockTickScheduler {
        &self.scheduler
    }
    
    /// Get a mutable reference to the scheduler
    pub fn scheduler_mut(&mut self) -> &mut BlockTickScheduler {
        &mut self.scheduler
    }
}