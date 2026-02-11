#![allow(unused)]

use crate::enums::ActorId;
use assets::map_setup::Category;
use enums::{Language, SpritePropId};
use logic::randomizer::Randomizer;
use serde::Deserialize;
use std::error::Error;
use strings::Strings;

mod assets;
mod data;
mod enums;
mod logic;
mod strings;
mod utils;

#[derive(Debug, Default, Deserialize)]
struct Config {
    #[serde(default)]
    actors: Vec<ActorId>,

    #[serde(default)]
    sprites: Vec<SpritePropId>,

    #[serde(default)]
    mix: bool,

    #[serde(default)]
    worlds: bool,

    #[serde(default)]
    moves: bool,

    #[serde(default)]
    notedoors: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let strbuf = std::fs::read_to_string("config.json").unwrap();
    let Ok(config) = serde_json::from_str::<Config>(&strbuf) else {
        eprintln!("config.json is malformed!");
        return Ok(());
    };

    let mut rando = Randomizer::new()?;

    if config.worlds {
        println!("shuffle worlds");
        rando.shuffle_world_order()?;
    }
    rando.remove_specific_actors()?;
    rando.change_randomizer_dialogues();

    println!("shuffle entities");
    if config.mix {
        rando.shuffle_entities(config.actors, config.sprites);
    } else {
        rando.shuffle_entities(config.actors, vec![]);
        rando.shuffle_entities(vec![], config.sprites);
    }

    if config.moves {
        rando.unlock_moves();
    }

    if config.notedoors {
        rando.remove_note_doors();
    }

    println!("write everything");
    rando.save()?;

    // check the rando can read what it wrote
    // let _rando = Randomizer::new()?;

    // For some reason, doesn't work (strings point to the wrong offset)
    // let mut reader = File::open("X360_strings.dat.bin")?;
    // let mut strings = Strings::new(&mut reader)?;

    // *strings.strings[167]
    //     .translations
    //     .get_mut(&Language::English)
    //     .unwrap() = "RANDOMIZER BY MINIROP".to_owned();
    // *strings.strings[167]
    //     .translations
    //     .get_mut(&Language::French)
    //     .unwrap() = "RANDOMIZER PAR MINIROP".to_owned();

    // let mut writer = File::create("X360_strings.dat")?;
    // strings.write(&mut writer)?;

    Ok(())
}
