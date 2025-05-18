use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::{Axolotl, AxolotlVariant, PlayDead};
use quill_common::components::{WaterBreathing, Tameable, BucketPickupable};
use uuid::Uuid;

/// Axolotl color variants
pub enum AxolotlColor {
    Lucy,
    Wild,
    Gold,
    Cyan,
    Blue,
}

impl From<AxolotlColor> for AxolotlVariant {
    fn from(color: AxolotlColor) -> Self {
        match color {
            AxolotlColor::Lucy => AxolotlVariant(0),
            AxolotlColor::Wild => AxolotlVariant(1),
            AxolotlColor::Gold => AxolotlVariant(2),
            AxolotlColor::Cyan => AxolotlVariant(3),
            AxolotlColor::Blue => AxolotlVariant(4),
        }
    }
}

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    
    // Generate a random variant with Blue being rare (1/1200)
    let variant = if rand::random::<f32>() < 0.00083 {
        AxolotlColor::Blue
    } else {
        match rand::random::<u8>() % 4 {
            0 => AxolotlColor::Lucy,
            1 => AxolotlColor::Wild,
            2 => AxolotlColor::Gold,
            _ => AxolotlColor::Cyan,
        }
    };

    builder
        .add(Axolotl)
        .add(EntityKind::Axolotl)
        .add(AxolotlVariant::from(variant))
        .add(PlayDead { activated: false, timer: 0 })
        .add(WaterBreathing { air_ticks: 6000, max_air: 6000 })
        .add(Tameable { tamed: false, owner: None })
        .add(BucketPickupable);
}