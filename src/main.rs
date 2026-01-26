#![allow(unused)]
use crate::enums::*;
use db360::ASSETS;
use std::error::Error;
use std::fs::OpenOptions;

mod db360;
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

    let mut next_id = 0u16;
    for asset in ASSETS {
        match asset {
            AssetId::Empty => {}
            AssetId::Animation(animation_id) => {
                assert_eq!(next_id, animation_id.into());
            }
            AssetId::Midi(midi_id) => {
                assert_eq!(next_id, midi_id.into());
            }
            AssetId::Model(model_id) => {
                assert_eq!(next_id, model_id.into());
            }
            AssetId::MapSetup(map_setup_id) => {
                assert_eq!(next_id, map_setup_id.into());
            }
            AssetId::Dialogue(dialogue_id) => {
                assert_eq!(next_id, dialogue_id.into());
            }
            AssetId::Credits(credits_id) => {
                assert_eq!(next_id, credits_id.into());
            }
            AssetId::Sprite(sprite_id) => {
                assert_eq!(next_id, sprite_id.into());
            }
            AssetId::Question(question_id) => {
                assert_eq!(next_id, question_id.into());
            }
            AssetId::Unknown(unknown_id) => {
                assert_eq!(next_id, unknown_id.into());
            }
        }
        next_id += 1;
    }
    assert_eq!(next_id, ASSETS.len() as u16);

    Ok(())
}
