#![allow(unused)]

use crate::enums::ActorId;
use assets::map_setup::Category;
use enums::{Language, SpritePropId};
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

    // rando.shuffle_world_order()?;
    rando.remove_specific_actors()?;

    let actors = vec![
        Category::Actor(ActorId::BlubbersGold),
        Category::Actor(ActorId::BlueJinjo),
        Category::Actor(ActorId::CollectableBluePresent),
        Category::Actor(ActorId::CollectableGreenPresent),
        Category::Actor(ActorId::CollectableRedPresent),
        Category::Actor(ActorId::EmptyHoneycomb),
        Category::Actor(ActorId::ExtraLife),
        Category::Actor(ActorId::GreenJinjo),
        Category::Actor(ActorId::Jiggy),
        Category::Actor(ActorId::MmmFlowerPot),
        Category::Actor(ActorId::MumboToken),
        Category::Actor(ActorId::OrangeJinjo),
        Category::Actor(ActorId::PinkJinjo),
        Category::Actor(ActorId::YellowJinjo),
    ];
    let sprites = vec![
        SpritePropId::BlueEgg,
        SpritePropId::GoldFeather,
        SpritePropId::MusicalNote,
        SpritePropId::RedFeather,
    ];
    rando.shuffle_entities(actors, sprites);

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
