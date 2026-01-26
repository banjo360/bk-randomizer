use crate::lvl_setup::Category;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use enums::ActorId;
use rand::prelude::SliceRandom;
use rand::rng;
use std::fs::OpenOptions;
use std::io::Seek;
use std::{error::Error, io::SeekFrom};

mod dialogue;
mod enums;
mod lvl_setup;
mod question;
mod strings;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    // need an unencrypted file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("db360.cmp")?;
    // 1821 = SM lvl setup (+1 for "number of elements")
    file.seek(SeekFrom::Start(1822 * 8))?;
    let addr = file.read_u32::<BigEndian>()?;
    file.seek(SeekFrom::Start(addr as u64 + 3702 * 8))?;
    let mut setup = lvl_setup::LevelSetup::new(&mut file)?;

    let mut objects = vec![];

    // an example how you can shuffle the honeycombs and extralives in SM
    for c in &setup.cubes {
        for o in &c.props_1 {
            if let Category::Actor(actor_id) = o.category {
                match actor_id {
                    ActorId::ExtraLife | ActorId::HoneyComb => objects.push(*o),
                    _ => break,
                }
            }
        }
    }

    objects.shuffle(&mut rng());

    let mut index = 0;
    for c in &mut setup.cubes {
        for o in &mut c.props_1 {
            if let Category::Actor(actor_id) = o.category {
                match actor_id {
                    ActorId::ExtraLife | ActorId::HoneyComb => {
                        // overwrite everything except the position
                        o.selector_or_radius = objects[index].selector_or_radius;
                        o.category = objects[index].category;
                        o.unk_bit_0 = objects[index].unk_bit_0;
                        o.marker_id = objects[index].marker_id;
                        o.byte_0b = objects[index].byte_0b;
                        o.bitfield_0c = objects[index].bitfield_0c;
                        o.bitfield_10 = objects[index].bitfield_10;
                        index += 1;
                    }
                    _ => break,
                }
            }
        }
    }

    file.seek(SeekFrom::Start(addr as u64 + 3702 * 8))?;
    setup.write(&mut file)?;

    Ok(())
}
