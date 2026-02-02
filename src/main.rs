use crate::data::levels::LevelOrder;
use logic::randomizer::Randomizer;
use rand::prelude::SliceRandom;
use rand::rng;
use std::error::Error;

mod assets;
mod data;
mod enums;
mod logic;
mod strings;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rando = Randomizer::new()?;

    let mut level_order = vec![
        LevelOrder::MumbosMountain,
        LevelOrder::TreasureTroveCove,
        LevelOrder::ClankersCavern,
        LevelOrder::BubbleGloopSwamp,
        LevelOrder::FreezeezyPeak,
        LevelOrder::GobisValley,
        LevelOrder::ClickClockWood,
        LevelOrder::RustyBucketBay,
        LevelOrder::MadMonsterMansion,
    ];
    level_order.shuffle(&mut rng());

    rando.set_world_order(level_order)?;

    rando.save()?;

    Ok(())
}
