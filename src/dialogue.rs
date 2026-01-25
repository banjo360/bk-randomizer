use crate::enum_builder;
use crate::utils::read_string;
use byteorder::BigEndian;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::io::Seek;

use crate::enums::Language;

enum_builder! {
    #[repr(u8)]
    pub enum Speaker {
        Banjo = 128,
        Kazooie = 129,
        Kazooie2 = 130,
        Bottles = 131,
        Mumbo = 132,
        Chimpy = 133,
        Conga = 134,
        Blubber = 135,
        Nipper = 136,
        Clanker = 137,
    }
}

#[derive(Debug)]
pub enum DialogueCommand {
    EndOfSection,
    SwitchBox,
    Speak(Speaker, String),
    Trigger(u8),
}

pub struct DialogueData {
    top: Vec<DialogueCommand>,
    bottom: Vec<DialogueCommand>,
}
pub struct Dialogue {
    scripts: HashMap<Language, DialogueData>,
}

impl Dialogue {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let languages = reader.read_u8()?;
        assert_eq!(languages, 4);

        let mut scripts = HashMap::new();

        for _ in 0..languages {
            // offsets
            reader.read_u16::<LittleEndian>()?;
        }

        for lang in 0..languages {
            let mut tops = vec![];
            let mut bottoms = vec![];

            let bottom = reader.read_u8()?;
            for _ in 0..bottom {
                let command = DialogueCommand::new(reader, lang.into())?;
                bottoms.push(command);
            }

            let top = reader.read_u8()?;
            for _ in 0..top {
                let command = DialogueCommand::new(reader, lang.into())?;
                tops.push(command);
            }

            scripts.insert(
                lang.into(),
                DialogueData {
                    top: tops,
                    bottom: bottoms,
                },
            );
        }

        Ok(Self { scripts })
    }
}

impl DialogueCommand {
    pub fn new<R: Read + Seek>(reader: &mut R, lang: Language) -> Result<Self, Box<dyn Error>> {
        let command_id = reader.read_u8()?;

        Ok(match command_id {
            4 => {
                let string = read_string(reader, lang)?;
                assert_eq!(string.len(), 0);
                DialogueCommand::EndOfSection
            }
            6 => {
                let string = read_string(reader, lang)?;
                assert_eq!(string.len(), 0);
                DialogueCommand::SwitchBox
            }
            7 => {
                let len = reader.read_u8()?;
                assert_eq!(len, 2);
                let value = reader.read_u8()?;

                let null = reader.read_u8()?;
                assert_eq!(null, 0);
                DialogueCommand::Trigger(value)
            }
            128..137 => {
                let string = read_string(reader, lang)?;
                DialogueCommand::Speak(command_id.into(), string)
            }
            _ => panic!("todo: command {command_id:X}"),
        })
    }
}
