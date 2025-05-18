use std::time::Duration;
use crate::{BlockKind, BlockProperties, BlockTickExecutor};
use base::{Chunk, ChunkPosition, ValidBlockPosition};
use blocks::BlockId;
use ahash::AHashMap;

/// Manages the integration of block systems with the world and chunk systems
pub struct BlockWorldIntegration {
    /// Block tick executor to handle block ticking behavior
    tick_executor: BlockTickExecutor,
    /// Tracks which chunks have been registered for ticking
    registered_chunks: AHashMap<ChunkPosition, bool>,
    /// Stores pending block updates to be processed
    pending_updates: Vec<BlockUpdate>,
    /// How often to process random ticks (in ticks)
    random_tick_interval: u32,
    /// Current tick count since server start
    current_tick: u64,
}

/// Represents a pending block update
struct BlockUpdate {
    position: ValidBlockPosition,
    kind: BlockKind,
    delay: u32,
    priority: i32,
}

impl BlockWorldIntegration {
    /// Creates a new block world integration system
    pub fn new(tick_executor: BlockTickExecutor) -> Self {
        Self {
            tick_executor,
            registered_chunks: AHashMap::new(),
            pending_updates: Vec::new(),
            random_tick_interval: 1, // Process random ticks every tick
            current_tick: 0,
        }
    }
    
    /// Registers a chunk for processing
    pub fn register_chunk(&mut self, pos: ChunkPosition) {
        self.registered_chunks.insert(pos, true);
    }
    
    /// Unregisters a chunk from processing
    pub fn unregister_chunk(&mut self, pos: ChunkPosition) {
        self.registered_chunks.remove(&pos);
    }
    
    /// Schedules a block update at the given position
    pub fn schedule_block_update(&mut self, pos: ValidBlockPosition, kind: BlockKind, delay: u32, priority: i32) {
        self.pending_updates.push(BlockUpdate {
            position: pos,
            kind,
            delay,
            priority,
        });
    }
    
    /// Main update method, to be called each game tick
    pub fn update<F, G>(&mut self, 
        block_getter: F, 
        block_setter: G,
        chunks: &AHashMap<ChunkPosition, Chunk>
    ) 
    where
        F: Fn(ValidBlockPosition) -> Option<(BlockKind, BlockProperties)>,
        G: FnMut(ValidBlockPosition, BlockKind),
    {
        self.current_tick += 1;
        
        // Process scheduled ticks
        self.tick_executor.process_ticks(block_getter, block_setter);
        
        // Process random ticks for registered chunks
        if self.current_tick % u64::from(self.random_tick_interval) == 0 {
            self.process_random_ticks(block_getter, block_setter, chunks);
        }
        
        // Process pending block updates
        self.process_pending_updates(block_getter);
    }
    
    /// Process random ticks for all registered chunks
    fn process_random_ticks<F, G>(
        &self,
        block_getter: F,
        mut block_setter: G,
        chunks: &AHashMap<ChunkPosition, Chunk>
    )
    where
        F: Fn(ValidBlockPosition) -> Option<(BlockKind, BlockProperties)>,
        G: FnMut(ValidBlockPosition, BlockKind),
    {
        for (pos, _) in &self.registered_chunks {
            if let Some(chunk) = chunks.get(pos) {
                // Collect ticking blocks in the chunk
                let mut ticking_blocks = Vec::new();
                for y in 0..chunk.height() {
                    for z in 0..16 {
                        for x in 0..16 {
                            let block_pos = ValidBlockPosition::new(
                                pos.x * 16 + x as i32,
                                y as i32,
                                pos.z * 16 + z as i32,
                            ).unwrap();
                            
                            if let Some((kind, properties)) = block_getter(block_pos) {
                                if kind.receives_random_ticks() {
                                    ticking_blocks.push((kind, block_pos, properties));
                                }
                            }
                        }
                    }
                }
                
                // Process random ticks for the chunk
                self.tick_executor.scheduler().process_random_ticks(
                    *pos,
                    &ticking_blocks,
                    block_getter,
                    block_setter,
                );
            }
        }
    }
    
    /// Process pending block updates
    fn process_pending_updates<F>(&mut self, block_getter: F) 
    where
        F: Fn(ValidBlockPosition) -> Option<(BlockKind, BlockProperties)>,
    {
        let current_tick = self.current_tick;
        let mut i = 0;
        
        while i < self.pending_updates.len() {
            let update = &self.pending_updates[i];
            if u64::from(update.delay) <= current_tick {
                let update = self.pending_updates.remove(i);
                
                // Check if the block still exists and is the same kind
                if let Some((current_kind, properties)) = block_getter(update.position) {
                    if current_kind == update.kind {
                        self.tick_executor.schedule_tick(
                            (update.position.x() as i32, update.position.y() as i32, update.position.z() as i32),
                            update.kind,
                            Duration::from_millis(0),
                            update.priority,
                        );
                    }
                }
            } else {
                i += 1;
            }
        }
    }
    
    /// Propagates block updates to neighboring blocks
    pub fn propagate_block_update<F, G>(
        &mut self,
        pos: ValidBlockPosition,
        block_getter: F,
        block_setter: G
    ) 
    where
        F: Fn(ValidBlockPosition) -> Option<(BlockKind, BlockProperties)>,
        G: FnMut(ValidBlockPosition, BlockKind),
    {
        // Get the neighboring positions
        let neighbors = [
            ValidBlockPosition::new(pos.x() + 1, pos.y(), pos.z()),
            ValidBlockPosition::new(pos.x() - 1, pos.y(), pos.z()),
            ValidBlockPosition::new(pos.x(), pos.y() + 1, pos.z()),
            ValidBlockPosition::new(pos.x(), pos.y() - 1, pos.z()),
            ValidBlockPosition::new(pos.x(), pos.y(), pos.z() + 1),
            ValidBlockPosition::new(pos.x(), pos.y(), pos.z() - 1),
        ];
        
        for neighbor_pos in neighbors.iter().flatten() {
            if let Some((kind, _)) = block_getter(*neighbor_pos) {
                // Schedule an update for the neighboring block
                self.schedule_block_update(*neighbor_pos, kind, 1, 0);
            }
        }
    }
    
    /// Handle a block being changed
    pub fn on_block_changed<F, G>(
        &mut self,
        pos: ValidBlockPosition,
        new_block: BlockKind,
        block_getter: F,
        mut block_setter: G
    ) 
    where
        F: Fn(ValidBlockPosition) -> Option<(BlockKind, BlockProperties)>,
        G: FnMut(ValidBlockPosition, BlockKind),
    {
        // Propagate changes to neighbors
        self.propagate_block_update(pos, &block_getter, &block_setter);
        
        // Check if the new block needs an initial tick
        if new_block.receives_random_ticks() {
            self.tick_executor.schedule_tick(
                (pos.x() as i32, pos.y() as i32, pos.z() as i32),
                new_block,
                Duration::from_millis(0),
                0,
            );
        }
    }
    
    /// Get a reference to the tick executor
    pub fn tick_executor(&self) -> &BlockTickExecutor {
        &self.tick_executor
    }
    
    /// Get a mutable reference to the tick executor
    pub fn tick_executor_mut(&mut self) -> &mut BlockTickExecutor {
        &mut self.tick_executor
    }
}