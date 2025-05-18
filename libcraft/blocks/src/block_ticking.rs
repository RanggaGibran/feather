use std::collections::{BinaryHeap, HashMap};
use std::cmp::{Ordering, Reverse};
use std::time::{Duration, Instant};
use rand::{Rng, thread_rng};

use crate::{BlockKind, BlockProperties};

/// Types of block ticks
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TickType {
    /// Random tick that occurs periodically based on random tick speed
    Random,
    /// Scheduled tick that occurs at a specific time
    Scheduled,
}

/// A scheduled tick for a block
#[derive(Debug, Clone)]
pub struct BlockTick {
    /// The position of the block (chunk-relative coordinates)
    pub position: (i32, i32, i32),
    /// The kind of block
    pub kind: BlockKind,
    /// When this tick should execute
    pub scheduled_time: Instant,
    /// The type of tick
    pub tick_type: TickType,
    /// Priority of the tick (lower values = higher priority)
    pub priority: i32,
}

impl PartialEq for BlockTick {
    fn eq(&self, other: &Self) -> bool {
        self.scheduled_time.eq(&other.scheduled_time) && self.priority.eq(&other.priority)
    }
}

impl Eq for BlockTick {}

impl PartialOrd for BlockTick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BlockTick {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.scheduled_time)
            .cmp(&Reverse(other.scheduled_time))
            .then_with(|| self.priority.cmp(&other.priority))
    }
}

/// Manages block ticks for a world
pub struct BlockTickScheduler {
    /// The queue of pending block ticks
    pending_ticks: BinaryHeap<BlockTick>,
    /// Map of position to block ticks to avoid duplicates
    position_to_tick: HashMap<(i32, i32, i32), BlockTick>,
    /// Random tick speed (how many random ticks per chunk section per game tick)
    random_tick_speed: u32,
}

impl BlockTickScheduler {
    /// Creates a new block tick scheduler
    pub fn new(random_tick_speed: u32) -> Self {
        Self {
            pending_ticks: BinaryHeap::new(),
            position_to_tick: HashMap::new(),
            random_tick_speed,
        }
    }

    /// Schedules a block tick
    pub fn schedule_tick(
        &mut self, 
        position: (i32, i32, i32), 
        kind: BlockKind, 
        delay: Duration,
        tick_type: TickType,
        priority: i32,
    ) {
        let scheduled_time = Instant::now() + delay;
        let tick = BlockTick {
            position,
            kind,
            scheduled_time,
            tick_type,
            priority,
        };

        // If a tick at this position already exists, replace it
        if let Some(existing_tick) = self.position_to_tick.remove(&position) {
            self.pending_ticks.iter()
                .position(|t| t.position == existing_tick.position)
                .map(|i| self.pending_ticks.into_iter().nth(i));
        }
        
        self.position_to_tick.insert(position, tick.clone());
        self.pending_ticks.push(tick);
    }

    /// Processes all ticks that are due
    pub fn process_ticks<F>(&mut self, mut tick_handler: F)
    where
        F: FnMut((i32, i32, i32), BlockKind, TickType),
    {
        let now = Instant::now();
        
        // Process all ticks that are due
        while let Some(tick) = self.pending_ticks.peek() {
            if tick.scheduled_time > now {
                break;
            }
            
            let tick = self.pending_ticks.pop().unwrap();
            self.position_to_tick.remove(&tick.position);
            
            tick_handler(tick.position, tick.kind, tick.tick_type);
        }
    }

    /// Performs random ticks in a chunk section
    pub fn process_random_ticks<F>(
        &self,
        chunk_position: (i32, i32),
        blocks: &[(BlockKind, (i32, i32, i32), BlockProperties)],
        mut tick_handler: F,
    )
    where
        F: FnMut((i32, i32, i32), BlockKind),
    {
        let mut rng = thread_rng();
        
        // Perform random_tick_speed random ticks
        for _ in 0..self.random_tick_speed {
            if blocks.is_empty() {
                return;
            }
            
            let index = rng.gen_range(0..blocks.len());
            let (kind, pos, _) = &blocks[index];
            
            tick_handler(*pos, *kind);
        }
    }

    /// Clear all pending ticks
    pub fn clear(&mut self) {
        self.pending_ticks.clear();
        self.position_to_tick.clear();
    }
    
    /// Set the random tick speed
    pub fn set_random_tick_speed(&mut self, speed: u32) {
        self.random_tick_speed = speed;
    }
    
    /// Get the random tick speed
    pub fn random_tick_speed(&self) -> u32 {
        self.random_tick_speed
    }
}