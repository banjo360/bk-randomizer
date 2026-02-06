#![allow(unused)]

use enums::Language;
use logic::randomizer::Randomizer;
use std::{error::Error, fs::File};
use strings::Strings;

mod assets;
mod data;
mod enums;
mod logic;
mod strings;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rando = Randomizer::new()?;

    rando.shuffle_world_order()?;
    rando.remove_specific_actors()?;

    rando.save()?;

    // check the rando can read what it wrote
    let _rando = Randomizer::new()?;

    // For some reason, doesn't work (strings point to the wrong offset)
    // let mut reader = File::open("X360_strings.dat.bin")?;
    // let mut strings = Strings::new(&mut reader)?;

    // *strings.strings[167]
    //     .translations
    //     .get_mut(&Language::English)
    //     .unwrap() = "RANDOMIZER BY MINIROP.".to_owned();
    // *strings.strings[167]
    //     .translations
    //     .get_mut(&Language::French)
    //     .unwrap() = "RANDOMIZER PAR MINIROP.".to_owned();

    // let mut writer = File::create("X360_strings.dat")?;
    // strings.write(&mut writer)?;

    Ok(())
}
