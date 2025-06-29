//! Entity implementations.
//!
//! Each entity should be implemented in a submodule of this module.
//! It should export a `build_default(&mut EntityBuilder)` function to
//! add default components for that entity.

use ecs::EntityBuilder;
use quill_common::{components::OnGround, entity_init::EntityInit};
use uuid::Uuid;

/// Adds default components shared between all entities.
fn build_default(builder: &mut EntityBuilder) {
    builder.add(Uuid::new_v4()).add(OnGround(true));
}

pub mod area_effect_cloud;
pub mod armor_stand;
pub mod arrow;
pub mod axolotl;
pub mod bat;
pub mod bee;
pub mod blaze;
pub mod boat;
pub mod cat;
pub mod cave_spider;
pub mod chest_minecart;
pub mod chicken;
pub mod cod;
pub mod command_block_minecart;
pub mod cow;
pub mod creeper;
pub mod dolphin;
pub mod donkey;
pub mod dragon_fireball;
pub mod drowned;
pub mod egg;
pub mod elder_guardian;
pub mod end_crystal;
pub mod ender_dragon;
pub mod ender_pearl;
pub mod enderman;
pub mod endermite;
pub mod evoker;
pub mod evoker_fangs;
pub mod experience_bottle;
pub mod experience_orb;
pub mod eye_of_ender;
pub mod falling_block;
pub mod fireball;
pub mod firework_rocket;
pub mod fishing_bobber;
pub mod fox;
pub mod furnace_minecart;
pub mod ghast;
pub mod giant;
pub mod glow_squid;
pub mod goat;
pub mod guardian;
pub mod hoglin;
pub mod hopper_minecart;
pub mod horse;
pub mod husk;
pub mod illusioner;
pub mod iron_golem;
pub mod item;
pub mod item_frame;
pub mod leash_knot;
pub mod lightning_bolt;
pub mod llama;
pub mod llama_spit;
pub mod magma_cube;
pub mod minecart;
pub mod mooshroom;
pub mod mule;
pub mod ocelot;
pub mod painting;
pub mod panda;
pub mod parrot;
pub mod phantom;
pub mod pig;
pub mod piglin;
pub mod piglin_brute;
pub mod pillager;
pub mod player;
pub mod polar_bear;
pub mod potion;
pub mod pufferfish;
pub mod rabbit;
pub mod ravager;
pub mod salmon;
pub mod sheep;
pub mod shulker;
pub mod shulker_bullet;
pub mod silverfish;
pub mod skeleton;
pub mod skeleton_horse;
pub mod slime;
pub mod small_fireball;
pub mod snow_golem;
pub mod snowball;
pub mod spawner_minecart;
pub mod spectral_arrow;
pub mod spider;
pub mod squid;
pub mod stray;
pub mod strider;
pub mod tnt;
pub mod tnt_minecart;
pub mod trader_llama;
pub mod trident;
pub mod tropical_fish;
pub mod turtle;
pub mod vex;
pub mod villager;
pub mod vindicator;
pub mod wandering_trader;
pub mod witch;
pub mod wither;
pub mod wither_skeleton;
pub mod wither_skull;
pub mod wolf;
pub mod zoglin;
pub mod zombie;
pub mod zombie_horse;
pub mod zombie_villager;
pub mod zombified_piglin;
pub mod metadata;
pub mod interactions;
pub mod pathfinding;
pub mod components;
pub mod spawning;

pub fn add_entity_components(builder: &mut EntityBuilder, init: &EntityInit) {
    match init {
        EntityInit::AreaEffectCloud => area_effect_cloud::build_default(builder),
        EntityInit::ArmorStand => armor_stand::build_default(builder),
        EntityInit::Arrow => arrow::build_default(builder),
        EntityInit::Bat => bat::build_default(builder),
        EntityInit::Bee => bee::build_default(builder),
        EntityInit::Blaze => blaze::build_default(builder),
        EntityInit::Boat => boat::build_default(builder),
        EntityInit::Cat => cat::build_default(builder),
        EntityInit::CaveSpider => cave_spider::build_default(builder),
        EntityInit::Chicken => chicken::build_default(builder),
        EntityInit::Cod => cod::build_default(builder),
        EntityInit::Cow => cow::build_default(builder),
        EntityInit::Creeper => creeper::build_default(builder),
        EntityInit::Dolphin => dolphin::build_default(builder),
        EntityInit::Donkey => donkey::build_default(builder),
        EntityInit::DragonFireball => dragon_fireball::build_default(builder),
        EntityInit::Drowned => drowned::build_default(builder),
        EntityInit::ElderGuardian => elder_guardian::build_default(builder),
        EntityInit::EndCrystal => end_crystal::build_default(builder),
        EntityInit::EnderDragon => ender_dragon::build_default(builder),
        EntityInit::Enderman => enderman::build_default(builder),
        EntityInit::Endermite => endermite::build_default(builder),
        EntityInit::Evoker => evoker::build_default(builder),
        EntityInit::EvokerFangs => evoker_fangs::build_default(builder),
        EntityInit::ExperienceOrb => experience_orb::build_default(builder),
        EntityInit::EyeOfEnder => eye_of_ender::build_default(builder),
        EntityInit::FallingBlock => falling_block::build_default(builder),
        EntityInit::FireworkRocket => firework_rocket::build_default(builder),
        EntityInit::Fox => fox::build_default(builder),
        EntityInit::Ghast => ghast::build_default(builder),
        EntityInit::Giant => giant::build_default(builder),
        EntityInit::Guardian => guardian::build_default(builder),
        EntityInit::Hoglin => hoglin::build_default(builder),
        EntityInit::Horse => horse::build_default(builder),
        EntityInit::Husk => husk::build_default(builder),
        EntityInit::Illusioner => illusioner::build_default(builder),
        EntityInit::IronGolem => iron_golem::build_default(builder),
        EntityInit::Item => item::build_default(builder),
        EntityInit::ItemFrame => item_frame::build_default(builder),
        EntityInit::Fireball => fireball::build_default(builder),
        EntityInit::LeashKnot => leash_knot::build_default(builder),
        EntityInit::LightningBolt => lightning_bolt::build_default(builder),
        EntityInit::Llama => llama::build_default(builder),
        EntityInit::LlamaSpit => llama_spit::build_default(builder),
        EntityInit::MagmaCube => magma_cube::build_default(builder),
        EntityInit::Minecart => minecart::build_default(builder),
        EntityInit::ChestMinecart => chest_minecart::build_default(builder),
        EntityInit::CommandBlockMinecart => command_block_minecart::build_default(builder),
        EntityInit::FurnaceMinecart => furnace_minecart::build_default(builder),
        EntityInit::HopperMinecart => hopper_minecart::build_default(builder),
        EntityInit::SpawnerMinecart => spawner_minecart::build_default(builder),
        EntityInit::TntMinecart => tnt_minecart::build_default(builder),
        EntityInit::Mule => mule::build_default(builder),
        EntityInit::Mooshroom => mooshroom::build_default(builder),
        EntityInit::Ocelot => ocelot::build_default(builder),
        EntityInit::Painting => painting::build_default(builder),
        EntityInit::Panda => panda::build_default(builder),
        EntityInit::Parrot => parrot::build_default(builder),
        EntityInit::Phantom => phantom::build_default(builder),
        EntityInit::Pig => pig::build_default(builder),
        EntityInit::Piglin => piglin::build_default(builder),
        EntityInit::PiglinBrute => piglin_brute::build_default(builder),
        EntityInit::Pillager => pillager::build_default(builder),
        EntityInit::PolarBear => polar_bear::build_default(builder),
        EntityInit::Tnt => tnt::build_default(builder),
        EntityInit::Pufferfish => pufferfish::build_default(builder),
        EntityInit::Rabbit => rabbit::build_default(builder),
        EntityInit::Ravager => ravager::build_default(builder),
        EntityInit::Salmon => salmon::build_default(builder),
        EntityInit::Sheep => sheep::build_default(builder),
        EntityInit::Shulker => shulker::build_default(builder),
        EntityInit::ShulkerBullet => shulker_bullet::build_default(builder),
        EntityInit::Silverfish => silverfish::build_default(builder),
        EntityInit::Skeleton => skeleton::build_default(builder),
        EntityInit::SkeletonHorse => skeleton_horse::build_default(builder),
        EntityInit::Slime => slime::build_default(builder),
        EntityInit::SmallFireball => small_fireball::build_default(builder),
        EntityInit::SnowGolem => snow_golem::build_default(builder),
        EntityInit::Snowball => snowball::build_default(builder),
        EntityInit::SpectralArrow => spectral_arrow::build_default(builder),
        EntityInit::Spider => spider::build_default(builder),
        EntityInit::Squid => squid::build_default(builder),
        EntityInit::Stray => stray::build_default(builder),
        EntityInit::Strider => strider::build_default(builder),
        EntityInit::Egg => egg::build_default(builder),
        EntityInit::EnderPearl => ender_pearl::build_default(builder),
        EntityInit::ExperienceBottle => experience_bottle::build_default(builder),
        EntityInit::Potion => potion::build_default(builder),
        EntityInit::Trident => trident::build_default(builder),
        EntityInit::TraderLlama => trader_llama::build_default(builder),
        EntityInit::TropicalFish => tropical_fish::build_default(builder),
        EntityInit::Turtle => turtle::build_default(builder),
        EntityInit::Vex => vex::build_default(builder),
        EntityInit::Villager => villager::build_default(builder),
        EntityInit::Vindicator => vindicator::build_default(builder),
        EntityInit::WanderingTrader => wandering_trader::build_default(builder),
        EntityInit::Witch => witch::build_default(builder),
        EntityInit::Wither => wither::build_default(builder),
        EntityInit::WitherSkeleton => wither_skeleton::build_default(builder),
        EntityInit::WitherSkull => wither_skull::build_default(builder),
        EntityInit::Wolf => wolf::build_default(builder),
        EntityInit::Zoglin => zoglin::build_default(builder),
        EntityInit::Zombie => zombie::build_default(builder),
        EntityInit::ZombieHorse => zombie_horse::build_default(builder),
        EntityInit::ZombieVillager => zombie_villager::build_default(builder),
        EntityInit::ZombifiedPiglin => zombified_piglin::build_default(builder),
        EntityInit::Player => player::build_default(builder),
        EntityInit::FishingBobber => fishing_bobber::build_default(builder),
        EntityInit::Axolotl => axolotl::build_default(builder),
        EntityInit::Goat => goat::build_default(builder),
        EntityInit::GlowSquid => glow_squid::build_default(builder),
    }
}

pub fn register(systems: &mut SystemExecutor<Game>) {
    behavior::register(systems);
    metadata::register(systems);
    interactions::register(systems);
    pathfinding::register(systems);
    spawning::register(systems);
    // Other registrations...
}
