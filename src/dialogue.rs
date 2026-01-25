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
        MutieSnippet = 138,
        MrVile = 139,
        ChoirMember = 140,
        Tanktup = 141,
        YellowFlibbit = 142,
        Trunker = 143,
        Rubee = 144,
        Gobi = 145,
        Grabba = 146,
        Napper = 147,
        YellowJinjo = 148,
        GreenJinjo = 149,
        BlueJinjo = 150,
        PinkJinjo = 151,
        OrangeJinjo = 152,
        Note = 153,
        Orange = 154,
        BlueEgg = 155,
        RedFeather = 156,
        GoldFeather = 157,
        Conga2 = 158,
        BlubbersGold = 159,
        Beehive = 160,
        EmptyHoneycomb = 161,
        ExtraLife = 162,
        Jiggy = 163,
        Beehive2 = 164,
        WadingBoots = 165,
        TurboTrainers = 166,
        BgsPiranha = 167,
        Ticker = 168,
        JuJu = 169,
        YumYum = 170,
        LittleLockup = 171,
        Leaky = 172,
        Gloop = 173,
        Tiptup = 174,
        Snacker = 175,
        Jinxy = 176,
        GvSandEel = 177,
        Snorkel = 178,
        AncientOnes = 179,
        Croctus = 180,
        Gruntilda = 181,
        Tooty = 182,
        Boggy = 183,
        Wozza = 184,
        Motzand = 185,
        Tumblar = 186,
        MumMum = 187,
        Present = 188,
        Caterpillar = 189,
        FpIceWater = 190,
        Twinklie = 191,
        TwinklieMuncher = 192,
        Gnawty = 193,
        BossBoomBox = 194,
        Zubba = 195,
        Nabnut = 196,
        BoggysKids = 197,
        BabyEyrie1 = 198,
        BabyEyrie2 = 199,
        BabyEyrie3 = 200,
        AdultEyrie = 201,
        Cauldron = 202,
        Brentilda = 203,
        Tooty2 = 204,
        BlackSnippet = 205,
        Loggo = 206,
        Cheato = 207,
        Present2 = 208,
        Present3 = 209,
        Klungo = 210,
        SexyGrunty = 211,
        UglyTooty = 212,
        Banjo2 = 213,
        Kazooie3 = 214,
        Tooty3 = 215,
        Dingpot = 216,
        MrVile2 = 217,
        Gruntilda2 = 218,
        Lockup = 219,
    }
}

#[derive(Debug)]
pub enum DialogueCommand {
    MrVileCheck,
    BottlesCheck,
    BoggyAndThirdCheatCheck,
    EndOfSection,
    SwitchBox,
    Trigger(u8),
    Selection(String),
    ItemCount,
    Speak(Speaker, String),
}

pub struct DialogueData {
    pub top: Vec<DialogueCommand>,
    pub bottom: Vec<DialogueCommand>,
}

pub struct Dialogue {
    pub scripts: HashMap<Language, DialogueData>,
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
                let command = DialogueCommand::new(reader)?;
                bottoms.push(command);
            }

            let top = reader.read_u8()?;
            for _ in 0..top {
                let command = DialogueCommand::new(reader)?;
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
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let command_id = reader.read_u8()?;

        Ok(match command_id {
            1 => {
                let string = read_string(reader)?;
                assert_eq!(string.len(), 0);
                DialogueCommand::MrVileCheck
            }
            2 => {
                let string = read_string(reader)?;
                assert_eq!(string.len(), 0);
                DialogueCommand::BottlesCheck
            }
            3 => {
                let string = read_string(reader)?;
                assert_eq!(string.len(), 0);
                DialogueCommand::BoggyAndThirdCheatCheck
            }
            4 => {
                let string = read_string(reader)?;
                assert_eq!(string.len(), 0);
                DialogueCommand::EndOfSection
            }
            6 => {
                let string = read_string(reader)?;
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
            8 => {
                let string = read_string(reader)?;
                DialogueCommand::Selection(string)
            }
            9 => {
                let string = read_string(reader)?;
                assert_eq!(string.len(), 0);
                DialogueCommand::ItemCount
            }
            128..=219 => {
                let string = read_string(reader)?;
                DialogueCommand::Speak(command_id.into(), string)
            }
            _ => panic!("todo: command {command_id:X} ({command_id})"),
        })
    }
}
