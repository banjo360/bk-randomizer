#![allow(unused)]
use crate::enums::*;
use crate::unknown::Unknown;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use db360::ASSETS;
use dialogue::Dialogue;
use map_setup::MapSetup;
use question::Question;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;

mod animation;
mod db360;
mod dialogue;
mod enums;
mod map_setup;
mod question;
mod strings;
mod unknown;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("db360.cmp")?;

    let entry_count = file.read_u32::<BigEndian>()?;
    assert_eq!(entry_count, 3701);
    let padding = file.read_u32::<BigEndian>()?;

    let mut sizes = vec![];

    let mut curr_offset = file.read_u32::<BigEndian>()?;
    let _ = file.read_u32::<BigEndian>()?;

    for _ in 1..entry_count {
        let offset = file.read_u32::<BigEndian>()?;
        let _ = file.read_u32::<BigEndian>()?;

        sizes.push((offset - curr_offset) as usize);
        curr_offset = offset;
    }

    let file_size = file.metadata()?.len() as usize;
    sizes.push(file_size - curr_offset as usize);

    assert_eq!(sizes.len(), entry_count as usize);

    // parse all (known) assets to check the readers
    for (id, asset) in ASSETS.iter().enumerate() {
        match asset {
            AssetId::Empty => {}
            AssetId::Animation(animation_id) => {
                Unknown::new(&mut file, sizes[id])?;
            }
            AssetId::Midi(midi_id) => {
                Unknown::new(&mut file, sizes[id])?;
            }
            AssetId::Model(model_id) => {
                Unknown::new(&mut file, sizes[id])?;
            }
            AssetId::MapSetup(map_setup_id) => {
                MapSetup::new(&mut file)?;
            }
            AssetId::Dialogue(dialogue_id) => {
                Dialogue::new(&mut file)?;
            }
            AssetId::Credits(credits_id) => {
                Dialogue::new(&mut file)?;
            }
            AssetId::Sprite(sprite_id) => {
                Unknown::new(&mut file, sizes[id])?;
            }
            AssetId::Question(question_id) => {
                Question::new(&mut file)?;
            }
            AssetId::Unknown(unknown_id) => {
                Unknown::new(&mut file, sizes[id])?;
            }
        }

        // for some reason, some files are aligned on 8 bytes and some aren't
        while let Ok(byte) = file.read_u8() {
            if byte != 0xCD {
                break;
            }
        }

        file.seek_relative(-1);
    }

    Ok(())
}
