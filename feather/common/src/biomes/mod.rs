pub mod integration;
pub mod entity_interaction;

pub use integration::BiomeIntegration;
pub use entity_interaction::BiomeEntityInteraction;

use ecs::SystemExecutor;
use crate::Game;

pub fn register(systems: &mut SystemExecutor<Game>) {
    integration::register(systems);
    entity_interaction::register(systems);
}