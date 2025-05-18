use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::{GlowSquid, GlowIntensity};
use quill_common::components::DropsItem;
use uuid::Uuid;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    
    builder
        .add(GlowSquid)
        .add(EntityKind::GlowSquid)
        .add(GlowIntensity { value: 1.0 })
        .add(DropsItem {
            item_type: "minecraft:glow_ink_sac",
            min_count: 1,
            max_count: 3,
            chance: 1.0,
        });
}