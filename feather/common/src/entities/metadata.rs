use base::{EntityKind, Metadata};
use ecs::{IntoQuery, SysResult, SystemExecutor};
use quill_common::entities::{Axolotl, AxolotlVariant, PlayDead, Goat, GlowSquid, GlowIntensity};
use quill_common::components::Health;

use crate::Game;

// Define metadata indices for the new entities
pub mod indices {
    // Axolotl metadata indices
    pub mod axolotl {
        pub const VARIANT: u8 = 17;
        pub const PLAYING_DEAD: u8 = 18;
    }
    
    // Goat metadata indices
    pub mod goat {
        pub const IS_SCREAMING: u8 = 17;
        pub const HAS_HORNS: u8 = 18;
    }
    
    // GlowSquid metadata indices
    pub mod glow_squid {
        pub const GLOW_INTENSITY: u8 = 17;
    }
}

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems
        .add_system(update_axolotl_metadata)
        .add_system(update_goat_metadata)
        .add_system(update_glow_squid_metadata);
}

fn update_axolotl_metadata(game: &mut Game) -> SysResult {
    for (_, (axolotl, variant, play_dead, metadata)) in game
        .ecs
        .query::<(&Axolotl, &AxolotlVariant, &PlayDead, &mut Metadata)>()
        .iter()
    {
        // Set variant metadata
        metadata.set_byte(indices::axolotl::VARIANT, variant.0 as i8);
        
        // Set playing dead metadata
        metadata.set_boolean(indices::axolotl::PLAYING_DEAD, play_dead.activated);
    }
    
    Ok(())
}

fn update_goat_metadata(game: &mut Game) -> SysResult {
    for (_, (goat, metadata)) in game
        .ecs
        .query::<(&Goat, &mut Metadata)>()
        .iter()
    {
        // Set is_screaming metadata
        metadata.set_boolean(indices::goat::IS_SCREAMING, goat.is_screaming);
        
        // Set has_horns metadata
        metadata.set_boolean(indices::goat::HAS_HORNS, goat.has_horns);
    }
    
    Ok(())
}

fn update_glow_squid_metadata(game: &mut Game) -> SysResult {
    for (_, (glow_squid, glow_intensity, metadata)) in game
        .ecs
        .query::<(&GlowSquid, &GlowIntensity, &mut Metadata)>()
        .iter()
    {
        // Set glow intensity as float metadata
        metadata.set_float(indices::glow_squid::GLOW_INTENSITY, glow_intensity.value);
    }
    
    Ok(())
}