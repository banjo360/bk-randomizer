#![allow(unused)]
use crate::assets::dialogue::Dialogue;
use crate::assets::map_setup::Category;
use crate::assets::map_setup::MapSetup;
use crate::assets::question::Question;
use crate::assets::unknown::Unknown;
use crate::data::xex::WORLD_OPENED_FLAGS;
use crate::data::xex::WORLD_SIGNS_FLAGS;
use crate::enums::*;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use data::db360::ASSETS;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;

mod assets;
mod data;
mod enums;
mod strings;
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
        let pos = file.seek(std::io::SeekFrom::Current(0))?;

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
                let mut map = MapSetup::new(&mut file)?;

                match map_setup_id {
                    MapSetupId::GlMmLobby => {
                        for c in &mut map.cubes {
                            for o in &mut c.props_1 {
                                if let Category::WarpOrTrigger(WarpOrTriggerId::MmEnterLevel) =
                                    o.category
                                {
                                    o.category =
                                        Category::WarpOrTrigger(WarpOrTriggerId::FpEnterLevel);
                                }

                                if let Category::Actor(ActorId::WorldSign) = o.category {
                                    o.selector_or_radius = 5;
                                }
                            }
                        }

                        file.seek(std::io::SeekFrom::Start(pos))?;
                        map.write(&mut file)?;
                    }
                    _ => {}
                }
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
            AssetId::Xbox(xbox_id) => {
                Dialogue::new(&mut file)?;
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

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("default.xex")?;
    file.seek(std::io::SeekFrom::Start(WORLD_OPENED_FLAGS + 4 * 4))?;
    file.write_u32::<BigEndian>(0x00690002)?;
    file.seek(std::io::SeekFrom::Start(WORLD_SIGNS_FLAGS + 4 * 2))?;
    file.write_u16::<BigEndian>(0x0031)?;

    Ok(())
}
