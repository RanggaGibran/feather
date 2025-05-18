use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::{Goat, GoatHorns, RammingCooldown};
use quill_common::components::JumpStrength;
use uuid::Uuid;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    
    // Determine if goat is screaming (rare variant)
    let is_screaming = rand::random::<f32>() < 0.02; // 2% chance
    
    // Determine if goat has horns (default is true)
    let has_horns = rand::random::<f32>() < 0.90; // 90% chance
    
    builder
        .add(Goat { is_screaming })
        .add(EntityKind::Goat)
        .add(GoatHorns { has_horns })
        .add(RammingCooldown { ticks: 0 })
        .add(JumpStrength(0.8));
}