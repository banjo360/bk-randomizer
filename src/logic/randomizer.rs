use crate::assets::Asset;
use crate::assets::animation::Animation;
use crate::assets::dialogue::Dialogue;
use crate::assets::map_setup::Category;
use crate::assets::map_setup::MapSetup;
use crate::assets::midi::Midi;
use crate::assets::model::Model;
use crate::assets::question::Question;
use crate::assets::sprite::Sprite;
use crate::assets::unknown::Unknown;
use crate::data::db360::ASSETS;
use crate::data::levels::LEVELS_INFO;
use crate::data::levels::LevelOrder;
use crate::data::xex::LAIR_WARPS_TARGET;
use crate::data::xex::MOLEHILLS_MOVES_DATA;
use crate::enums::*;
use crate::utils::align_writer;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use rand::prelude::SliceRandom;
use rand::rng;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;
use std::ops::Index;
use std::ops::IndexMut;

struct AssetData {
    asset: Asset,
    flag: u32,
}

struct TextureData {
    address: u32,
    edited: u32,
}

impl Index<textures::TextureId> for Vec<TextureData> {
    type Output = TextureData;
    fn index(&self, index: textures::TextureId) -> &Self::Output {
        let i: usize = index.into();
        &self[i]
    }
}

impl IndexMut<textures::TextureId> for Vec<TextureData> {
    fn index_mut(&mut self, index: textures::TextureId) -> &mut Self::Output {
        let i: usize = index.into();
        &mut self[i]
    }
}

impl Index<&textures::TextureId> for Vec<TextureData> {
    type Output = TextureData;
    fn index(&self, index: &textures::TextureId) -> &Self::Output {
        let i: usize = (*index).into();
        &self[i]
    }
}

impl IndexMut<&textures::TextureId> for Vec<TextureData> {
    fn index_mut(&mut self, index: &textures::TextureId) -> &mut Self::Output {
        let i: usize = (*index).into();
        &mut self[i]
    }
}

pub struct Randomizer {
    assets: Vec<AssetData>,
    textures: Vec<TextureData>,
}

impl Randomizer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let assets = read_db360()?;
        let textures = read_textures()?;

        Ok(Self { assets, textures })
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        self.write_db360()?;
        self.write_textures()?;

        Ok(())
    }

    pub fn shuffle_world_order(&mut self) -> Result<(), Box<dyn Error>> {
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

        loop {
            level_order.shuffle(&mut rng());

            // since the first world need talon trot
            if level_order[0].has_molehill() {
                break;
            }

            // need to check shock jump too
            // if level 1 is CC, GV or FP
            // and levels 2, 3, 4 are CCW, RBB, MMM
            // then all molehills are after the spring pad
        }

        self.set_world_order(level_order.clone())?;
        self.shuffle_molehills(level_order)?;

        Ok(())
    }

    fn shuffle_molehills(&mut self, order: Vec<LevelOrder>) -> Result<(), Box<dyn Error>> {
        let mut molehills = vec![];
        for level in &order {
            for mole in LEVELS_INFO[*level].molehills {
                molehills.push(mole);
            }
        }

        let talon_trot_max_pos = LEVELS_INFO[order[0]].molehills.len();

        loop {
            molehills.shuffle(&mut rng());

            // since the first world need talon trot
            if molehills[0..talon_trot_max_pos]
                .iter()
                .any(|m| m.ability == Ability::TalonTrot)
            {
                break;
            }

            // need to check shock jump too
        }

        let mut xex = OpenOptions::new()
            .read(true)
            .write(true)
            .open("default.xex")?;

        let mut mole_index = 0;
        for level in &order {
            for mole in LEVELS_INFO[*level].molehills {
                xex.seek(SeekFrom::Start(
                    MOLEHILLS_MOVES_DATA + mole.table_index as u64 * 6, /* or 8? */
                ))?;

                xex.write_u16::<BigEndian>(molehills[mole_index].teach_text_id.into())?;
                xex.write_u16::<BigEndian>(molehills[mole_index].refresher_text_id.into())?;
                xex.read_u8()?; // easier than "skip"
                xex.write_u8(molehills[mole_index].ability.into())?;
                mole_index += 1;
            }
        }

        Ok(())
    }

    fn set_world_order(&mut self, order: Vec<LevelOrder>) -> Result<(), Box<dyn Error>> {
        let mut order = order;
        order.insert(LevelOrder::Lair.into(), LevelOrder::Lair);

        for (id, level) in order.iter().enumerate() {
            self.set_level_art(id.into(), *level);
            self.change_level_warp(id.into(), *level)?;
        }

        Ok(())
    }

    fn set_level_art(&mut self, old: LevelOrder, new: LevelOrder) {
        if old == LevelOrder::Lair {
            return;
        }

        let old_level = &LEVELS_INFO[old];
        let new_level = &LEVELS_INFO[new];

        for (o, n) in old_level.painting.iter().zip(new_level.painting) {
            self.textures[o].edited = self.textures[n].address;
        }

        self.textures[old_level.sign_left].edited = self.textures[new_level.sign_left].address;
        self.textures[old_level.sign_right].edited = self.textures[new_level.sign_right].address;

        for id in 0..4 {
            self.textures[old_level.label[id]].edited = self.textures[new_level.label[id]].address;
        }
    }

    fn change_level_warp(
        &mut self,
        old: LevelOrder,
        new: LevelOrder,
    ) -> Result<(), Box<dyn Error>> {
        if old == LevelOrder::Lair {
            return Ok(());
        }

        let old_level = &LEVELS_INFO[old];
        let new_level = &LEVELS_INFO[new];

        let map_setup: u16 = old_level.warp_lair.map_setup.into();
        let asset = &mut self.assets[map_setup as usize].asset;

        if let Asset::MapSetup(map_setup) = asset {
            for c in &mut map_setup.cubes {
                for o in &mut c.props_1 {
                    if let Category::WarpOrTrigger(id) = o.category {
                        if id == old_level.warp_lair.warp_id {
                            o.category = Category::WarpOrTrigger(new_level.warp_lair.warp_id);
                        }
                    }
                }
            }
        }

        let mut xex = OpenOptions::new().write(true).open("default.xex")?;
        let new_order: usize = new.into();
        xex.seek(SeekFrom::Start(LAIR_WARPS_TARGET + new_order as u64 * 4))?;
        xex.write_u16::<BigEndian>(old_level.warp_lair.map_id.into())?;
        xex.write_u16::<BigEndian>(old_level.warp_lair.exit_id)?;

        Ok(())
    }

    pub fn remove_specific_actors(&mut self) -> Result<(), Box<dyn Error>> {
        for asset in &mut self.assets {
            match &mut asset.asset {
                Asset::Animation(_animation) => {}
                Asset::Dialogue(_dialogue) => {}
                Asset::MapSetup(map_setup) => {
                    for c in &mut map_setup.cubes {
                        c.props_1.retain(|o| {
                            if let Category::Actor(actor_id) = o.category {
                                match actor_id {
                                    ActorId::Unknown(id) => id != 0x373,
                                    _ => true,
                                }
                            } else {
                                true
                            }
                        });
                    }
                }
                Asset::Question(_question) => {}
                Asset::Unknown(_unknown) => {}
                Asset::Sprite(_sprite) => {}
                Asset::Model(_model) => {}
                Asset::Midi(_midi) => {}
                Asset::Empty => {}
            }
        }

        Ok(())
    }

    fn write_db360(&self) -> Result<(), Box<dyn Error>> {
        let entry_count = self.assets.len();

        let mut patched = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("db360.cmp")?;

        patched.write_u32::<BigEndian>(entry_count as u32)?;
        patched.write_u32::<BigEndian>(0xCDCDCDCD)?;
        for _ in 0..entry_count {
            patched.write_u32::<BigEndian>(0)?;
            patched.write_u32::<BigEndian>(0)?;
        }

        let mut offsets = vec![];
        let header = patched.seek(SeekFrom::Current(0))?;

        for i in 0..entry_count {
            let current_offset = patched.seek(SeekFrom::Current(0))? - header;
            offsets.push(current_offset as u32);

            match &self.assets[i].asset {
                Asset::Animation(animation) => {
                    animation.write(&mut patched)?;
                }
                Asset::Dialogue(dialogue) => {
                    dialogue.write(&mut patched)?;
                }
                Asset::MapSetup(map_setup) => {
                    map_setup.write(&mut patched)?;
                }
                Asset::Question(question) => {
                    question.write(&mut patched)?;
                }
                Asset::Sprite(sprite) => {
                    sprite.write(&mut patched)?;
                }
                Asset::Model(model) => {
                    model.write(&mut patched)?;
                }
                Asset::Midi(midi) => {
                    midi.write(&mut patched)?;
                }
                Asset::Unknown(unknown) => {
                    unknown.write(&mut patched)?;
                }
                Asset::Empty => {}
            }

            align_writer(&mut patched)?;
        }

        patched.seek(SeekFrom::Start(8))?;

        for id in 0..ASSETS.len() {
            patched.write_u32::<BigEndian>(offsets[id])?;
            patched.write_u32::<BigEndian>(self.assets[id].flag)?;
        }

        Ok(())
    }

    fn write_textures(&self) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new().write(true).open("db360.textures.cmp")?;
        let metadata_size = 20;

        file.seek_relative(4)?;

        for data in &self.textures {
            file.write_u32::<BigEndian>(data.edited)?;
            file.seek_relative(metadata_size - 4)?;
        }

        Ok(())
    }
}

fn read_db360() -> Result<Vec<AssetData>, Box<dyn Error>> {
    let mut file = File::open("db360.cmp")?;

    let entry_count = file.read_u32::<BigEndian>()?;
    assert_eq!(entry_count, 3701);
    let _padding = file.read_u32::<BigEndian>()?;

    let mut sizes = vec![];
    let mut flags = vec![];

    let mut curr_offset = file.read_u32::<BigEndian>()?;
    let flag = file.read_u32::<BigEndian>()?;
    flags.push(flag);

    for _ in 1..entry_count {
        let offset = file.read_u32::<BigEndian>()?;
        let flag = file.read_u32::<BigEndian>()?;
        flags.push(flag);

        sizes.push((offset - curr_offset) as usize);
        curr_offset = offset;
    }

    let file_size = file.metadata()?.len() as usize;
    sizes.push(file_size - curr_offset as usize);

    assert_eq!(sizes.len(), entry_count as usize);

    let mut loaded_assets = vec![];

    for (id, asset) in ASSETS.iter().enumerate() {
        let _pos = file.seek(SeekFrom::Current(0))?;

        let asset = match asset {
            AssetId::Empty => Asset::Empty,
            AssetId::Animation(_animation_id) => {
                let data = Animation::new(&mut file)?;
                Asset::Animation(data)
            }
            AssetId::Midi(_midi_id) => {
                let data = Midi::new(&mut file, sizes[id])?;
                Asset::Midi(data)
            }
            AssetId::Model(_model_id) => {
                let data = Model::new(&mut file, sizes[id])?;
                Asset::Model(data)
            }
            AssetId::MapSetup(_map_setup_id) => {
                let map = MapSetup::new(&mut file)?;
                Asset::MapSetup(map)
            }
            AssetId::Dialogue(_dialogue_id) => {
                let data = Dialogue::new(&mut file)?;
                Asset::Dialogue(data)
            }
            AssetId::Credits(_credits_id) => {
                let data = Dialogue::new(&mut file)?;
                Asset::Dialogue(data)
            }
            AssetId::Sprite(sprite_id) => {
                match sprite_id {
                    // these sprites have a "strange" format (closer to N64 or something else)
                    SpriteId::Sprite0064D520
                    | SpriteId::Sprite0064E8D8
                    | SpriteId::Sprite0064EB58
                    | SpriteId::Sprite006536C8
                    | SpriteId::Sprite006546D8
                    | SpriteId::Sprite0064ECC8 => {
                        let data = Unknown::new(&mut file, sizes[id])?;
                        Asset::Unknown(data)
                    }
                    _ => {
                        let data = Sprite::new(&mut file)?;
                        Asset::Sprite(data)
                    }
                }
            }
            AssetId::Question(_question_id) => {
                let data = Question::new(&mut file)?;
                Asset::Question(data)
            }
            AssetId::Xbox(_xbox_id) => {
                let data = Dialogue::new(&mut file)?;
                Asset::Dialogue(data)
            }
            AssetId::Unknown(_unknown_id) => {
                let data = Unknown::new(&mut file, sizes[id])?;
                Asset::Unknown(data)
            }
        };

        loaded_assets.push(AssetData {
            asset,
            flag: flags[id],
        });

        // for some reason, some files are aligned on 8 bytes and some aren't
        while let Ok(byte) = file.read_u8() {
            if byte != 0xCD {
                break;
            }
        }

        file.seek_relative(-1)?;
    }

    assert_eq!(entry_count as usize, loaded_assets.len());

    Ok(loaded_assets)
}

fn read_textures() -> Result<Vec<TextureData>, Box<dyn Error>> {
    let mut file = File::open("db360.textures.cmp")?;
    let entry_count = file.read_u32::<BigEndian>()?;
    assert_eq!(entry_count, 6576);
    let metadata_size = 20;

    let mut addresses = vec![];
    for _ in 0..entry_count {
        let address = file.read_u32::<BigEndian>()?;
        addresses.push(TextureData {
            address,
            edited: address,
        });
        file.seek_relative(metadata_size - 4)?;
    }

    Ok(addresses)
}
