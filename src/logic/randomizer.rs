use super::enums::Props;
use super::location::Location;
use crate::Config;
use crate::assets::Asset;
use crate::assets::animation::Animation;
use crate::assets::dialogue::Dialogue;
use crate::assets::dialogue::DialogueCommand;
use crate::assets::dialogue::Speaker;
use crate::assets::map_setup::Category;
use crate::assets::map_setup::MapSetup;
use crate::assets::map_setup::Prop1;
use crate::assets::map_setup::Prop2;
use crate::assets::midi::Midi;
use crate::assets::model::Model;
use crate::assets::question::Question;
use crate::assets::sprite::Sprite;
use crate::assets::unknown::Unknown;
use crate::data::db360::ASSETS;
use crate::data::levels::LAIR_MAPS;
use crate::data::levels::LEVELS_INFO;
use crate::data::levels::LevelInfo;
use crate::data::levels::LevelOrder;
use crate::data::powerpc::Functions;
use crate::data::powerpc::call;
use crate::data::powerpc::epilogue;
use crate::data::powerpc::jump;
use crate::data::powerpc::nop;
use crate::data::powerpc::prologue;
use crate::data::powerpc::set_flag;
use crate::data::powerpc::set_flags;
use crate::data::xex::CODE_START_CUSTOM_ADDRESS;
use crate::data::xex::LAIR_WARPS_TARGET;
use crate::data::xex::MOLEHILLS_MOVES_DATA;
use crate::enums::*;
use crate::logic::randomizer::file_progress::FileProgress;
use crate::utils::Vector3;
use crate::utils::align_writer;
use byteorder::BigEndian;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use rand::Rng;
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

    pub fn shuffle_world_order(&mut self, config: &Config) -> Result<(), Box<dyn Error>> {
        println!("shuffle worlds");

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

        if config.moves {
            // if all moves are unlocked, the order doesn't matter
            level_order.shuffle(&mut rng());
        } else {
            // the first world need talon trot (but can't be GV)
            level_order[..5].shuffle(&mut rng());

            loop {
                level_order[1..].shuffle(&mut rng());

                let l0 = level_order[0].molehill_count(); // MM
                let l1 = level_order[1].molehill_count(); // TTC
                let l2 = level_order[2].molehill_count(); // CC
                let l3 = level_order[3].molehill_count(); // BGS

                // need beak buster before CC and shock jump before FP
                if (l0 + l1 >= 2 || config.pipes) && l0 + l1 + l2 + l3 >= 3 {
                    break;
                }
            }

            println!("shuffle molehills");
            self.shuffle_molehills(level_order.clone())?;
        }

        self.set_world_order(level_order.clone())?;
        println!("replace dialogues");
        self.replace_dialogues(level_order)?;

        Ok(())
    }

    fn get_map_setup(&mut self, map_setup_id: MapSetupId) -> &mut MapSetup {
        let setup_id: u16 = map_setup_id.into();
        if let Some(asset_data) = self.assets.get_mut(setup_id as usize) {
            if let Asset::MapSetup(map_setup) = &mut asset_data.asset {
                return map_setup;
            }
        }

        unreachable!();
    }

    pub fn fix_ttc_blue_egg(&mut self) {
        let map_setup = self.get_map_setup(MapSetupId::TreasureTroveCove);

        for cube in map_setup.cubes.iter_mut() {
            for mut entity in cube.props_2.iter_mut() {
                if let Prop2::Sprite {
                    id: _,
                    flags: _,
                    position,
                    bitfield_0a: _,
                } = &mut entity
                {
                    if *position
                        == (Vector3 {
                            x: -3976,
                            y: 1054,
                            z: 1750,
                        })
                    {
                        println!("fix blue egg 1");
                        position.y = 1190;
                        position.z = 1794;
                    } else if *position
                        == (Vector3 {
                            x: -3966,
                            y: 1190,
                            z: 1747,
                        })
                    {
                        println!("fix blue egg 2");
                        position.z = 1706;
                    }
                }
            }
        }
    }

    pub fn remove_bridge_molehill(&mut self) {
        let map_setup = self.get_map_setup(MapSetupId::SpiralMountain);

        for cube in map_setup.cubes.iter_mut() {
            cube.props_1.retain(|p| {
                if let Category::Actor(id) = p.category {
                    match id {
                        ActorId::SpiralMountainBottlesMolehill => p.selector_or_radius != 8,
                        ActorId::SpiralMountainBridgeForcedMovementStart => false,
                        ActorId::SpiralMountainBridgeForcedMovementTarget => false,
                        _ => true,
                    }
                } else if let Category::CameraController(id) = p.category {
                    id != 20
                } else {
                    true
                }
            });
        }
    }

    pub fn patch_code(&self, config: &Config) -> Result<(), Box<dyn Error>> {
        let mut xex = OpenOptions::new()
            .read(true)
            .write(true)
            .open("default.xex")
            .expect("Can't open default.xex, missing?");

        let custom_offset_start = xex.seek(SeekFrom::Start(CODE_START_CUSTOM_ADDRESS))?;

        let custom_address_start: u32 = Functions::CustomFunction.into();

        prologue(&mut xex)?;

        println!("remove flags");

        // remove "first time" flags
        // collectibles, meet mumbo, touched icy water, etc.
        set_flags(
            &mut xex,
            FileProgress::MusicNoteText,
            16,
            custom_address_start,
        )?;
        set_flag(
            &mut xex,
            FileProgress::HasTouchedFpIcyWater,
            custom_address_start,
        )?;
        set_flags(
            &mut xex,
            FileProgress::StoodOnJigsawPodium,
            2,
            custom_address_start,
        )?;
        set_flag(
            &mut xex,
            FileProgress::HasTouchedMmmThornHedge,
            custom_address_start,
        )?;
        set_flags(
            &mut xex,
            FileProgress::NearPuzzlePodiumText,
            6,
            custom_address_start,
        )?;
        set_flag(
            &mut xex,
            FileProgress::HasTouchedCcwIcyWater,
            custom_address_start,
        )?;
        set_flags(
            &mut xex,
            FileProgress::CanRemoveAllPuzzlePieces,
            2,
            custom_address_start,
        )?;

        // has entered levels
        set_flags(
            &mut xex,
            FileProgress::HasEnteredMm,
            9,
            custom_address_start,
        )?;

        // skip lair cutscene
        set_flag(
            &mut xex,
            FileProgress::EnterLairCutscene,
            custom_address_start,
        )?;

        // FF flags (met dingpot, saw FF cutscene, etc)
        set_flags(&mut xex, FileProgress::MetDingpot, 4, custom_address_start)?;

        // lair flags (met brentilda, pass 50 note door, etc)
        set_flags(
            &mut xex,
            FileProgress::MetBrentilda,
            4,
            custom_address_start,
        )?;

        if config.pipes {
            set_flags(
                &mut xex,
                FileProgress::LairGrateToBgsPuzzleOpen,
                4,
                custom_address_start,
            )?;
        }

        if config.cauldrons {
            set_flags(
                &mut xex,
                FileProgress::PinkCauldron1Active,
                10,
                custom_address_start,
            )?;
        }

        if config.skip_furnace_fun {
            set_flag(
                &mut xex,
                FileProgress::FurnaceFunComplete,
                custom_address_start,
            )?;
        }

        let current_custom_offset = xex.seek(SeekFrom::Current(0))?;
        let offset = (current_custom_offset - custom_offset_start) as u32;

        xex.write_u32::<BigEndian>(call(
            custom_address_start + offset,
            Functions::ChSmBottlesSkipIntroTutorial,
        ))?;

        if config.moves {
            println!("unlock moves");

            // li r3, -1
            xex.write_u32::<BigEndian>(0x3860ffff)?;

            let current_custom_offset = xex.seek(SeekFrom::Current(0))?;
            let offset = (current_custom_offset - custom_offset_start) as u32;

            xex.write_u32::<BigEndian>(call(
                custom_address_start + offset,
                Functions::AbilitySetAllLearned,
            ))?;
        }

        println!("open requested note doors");

        for cost in &config.notedoors {
            let flag = get_door_flag(*cost);
            set_flag(&mut xex, flag, custom_address_start)?;
        }

        epilogue(&mut xex)?;

        let current_custom_offset = xex.seek(SeekFrom::Current(0))?;
        let custom_size = (current_custom_offset - custom_offset_start) as u32;

        // increase size of .text section
        xex.seek(SeekFrom::Start(0x2248))?;
        xex.write_u32::<LittleEndian>(0x3b0cf4 + custom_size)?;

        // patch chSmBottles_update
        xex.seek(SeekFrom::Start(0x18da14))?;
        xex.write_u32::<BigEndian>(call(0x8218ba14, Functions::CustomFunction))?;

        // patch stoodOnPodiumCallback to skip bottles' instructions
        xex.seek(SeekFrom::Start(0x1826fc))?;
        xex.write_u32::<BigEndian>(0x38800004)?;

        // patch __baMarker_8028B848 to remove
        // - DIALOG_FIRST_JIGGY
        // - DIALOG_JIGGY_COLLECT_10
        xex.seek(SeekFrom::Start(0x94068))?;
        xex.write_u32::<BigEndian>(jump(0x82092068, 0x820920e8))?;

        if config.easy_talon_trot {
            println!("easy talon trot");

            // doesn't work
            // xex.seek(SeekFrom::Start(0xadb7c))?;
            // xex.write_u32::<BigEndian>(call(0x820abb7c, Functions::KeyPressed))?;

            // xex.seek(SeekFrom::Start(0x1e174))?;
            // xex.write_u32::<BigEndian>(call(0x820ac174, Functions::KeyPressed))?;
        }

        println!("globaliser");

        xex.seek(SeekFrom::Start(0x155844))?;
        for _ in 0..13 {
            nop(&mut xex)?;
        }

        for _ in 0..11 {
            xex.read_u32::<BigEndian>()?; // skip
            nop(&mut xex)?;
        }

        xex.seek(SeekFrom::Start(0x15591c))?;
        nop(&mut xex)?;

        Ok(())
    }

    fn replace_dialogues(&mut self, order: Vec<LevelOrder>) -> Result<(), Box<dyn Error>> {
        let mut order = order;
        order.insert(LevelOrder::Lair.into(), LevelOrder::Lair);

        for lang_id in 0..4 {
            let lang: Language = lang_id.into();

            // skip JP for now
            if lang == Language::Japanese {
                continue;
            }

            for asset in &mut self.assets {
                match &mut asset.asset {
                    Asset::Dialogue(dialogue) => {
                        let mut level_replaced = false;
                        for (id, level) in order.iter().enumerate() {
                            if *level == LevelOrder::Lair {
                                continue;
                            }

                            let orig_level: LevelOrder = id.into();

                            if orig_level == *level {
                                // level didn't change
                                continue;
                            }

                            let orig_level_name = orig_level.get_name(lang);
                            let level_name = level.get_name(lang);

                            if let Some(dial) = dialogue.translations.get_mut(&lang) {
                                for t in dial.top.iter_mut() {
                                    if let DialogueCommand::Speak(_, text) = t {
                                        if text.contains(orig_level_name) {
                                            *text = text.replace(orig_level_name, level_name);
                                            *text = text.replace("DE LE", "DU");
                                            level_replaced = true;
                                        }
                                    }
                                }

                                for b in dial.bottom.iter_mut() {
                                    if let DialogueCommand::Speak(_, text) = b {
                                        if text.contains(orig_level_name) {
                                            *text = text.replace(orig_level_name, level_name);
                                            *text = text.replace("DE LE", "DU");
                                            level_replaced = true;
                                        }
                                    }
                                }
                            }

                            // when A becomes B
                            // and B becomes C
                            // don't change A into C
                            if level_replaced {
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // shorter bottles' dialogues
        const MOVES_NAMES_DIALOGUES: [(DialogueId, &'static str); 9] = [
            (DialogueId::BottlesLearningEggs, "EGGS"),
            (DialogueId::BottlesLearningBeakBuster, "BEAK BUSTER"),
            (DialogueId::BottlesLearningTalonTrot, "TALON TROT"),
            (DialogueId::BottlesLearningShockJump, "SHOCK JUMP"),
            (DialogueId::BottlesLearningFlight, "FLIGHT"),
            (DialogueId::BottlesLearningWonderwing, "WONDERWING"),
            (DialogueId::BottlesLearningWadingBoots, "WADING BOOTS"),
            (DialogueId::BottlesLearningBeakBomb, "BEAK BOMB"),
            (DialogueId::BottlesLearningTurboTalon, "TURBO TALON TROT"),
        ];

        for (dial_id, name) in MOVES_NAMES_DIALOGUES {
            self.set_dialogue(
                dial_id,
                vec![DialogueCommand::EndOfSection],
                vec![
                    DialogueCommand::Speak(Speaker::Bottles, name.into()),
                    DialogueCommand::EndOfSection,
                ],
                Language::English,
            );
        }

        Ok(())
    }

    pub fn change_randomizer_dialogues(&mut self) {
        self.set_dialogue(
            DialogueId::BottlesIntro,
            vec![
                DialogueCommand::SwitchBox,
                DialogueCommand::Speak(Speaker::Banjo, "WHAT IS THAT?".into()),
                DialogueCommand::Speak(Speaker::Kazooie, "THINGS ARE ALL SHUFFLED RANDOMLY".into()),
                DialogueCommand::Speak(Speaker::Banjo, "SOUNDS INTERESTING!".into()),
                DialogueCommand::EndOfSection,
            ],
            vec![
                DialogueCommand::Speak(Speaker::Bottles, "WELCOME TO THE RANDOMIZER".into()),
                DialogueCommand::SwitchBox,
                DialogueCommand::EndOfSection,
            ],
            Language::English,
        );

        self.set_dialogue(
            DialogueId::BottlesIntroQuestion,
            vec![DialogueCommand::SwitchBox, DialogueCommand::EndOfSection],
            vec![
                DialogueCommand::Speak(Speaker::Bottles, "YOU KNOW THE GAME, JUST PRESS B.".into()),
                DialogueCommand::EndOfSection,
            ],
            Language::English,
        );

        self.set_dialogue(
            DialogueId::BottlesIntroTutorialSkipped,
            vec![DialogueCommand::SwitchBox, DialogueCommand::EndOfSection],
            vec![
                DialogueCommand::Speak(Speaker::Bottles, "OK, NOW GET LOST!".into()),
                DialogueCommand::EndOfSection,
            ],
            Language::English,
        );

        self.set_dialogue(
            DialogueId::BottlesTopOfSpiralMountainTutorialSkipped,
            vec![DialogueCommand::SwitchBox, DialogueCommand::EndOfSection],
            vec![
                DialogueCommand::Speak(Speaker::Bottles, "YOU FAILED THE SKIP. BOO! LOSER!".into()),
                DialogueCommand::EndOfSection,
            ],
            Language::English,
        );
    }

    fn set_dialogue(
        &mut self,
        id: DialogueId,
        top: Vec<DialogueCommand>,
        bottom: Vec<DialogueCommand>,
        lang: Language,
    ) {
        let dial_id: u16 = id.into();
        if let Some(asset_data) = self.assets.get_mut(dial_id as usize) {
            if let Asset::Dialogue(dialogue) = &mut asset_data.asset {
                let data = dialogue.translations.get_mut(&lang).unwrap();
                data.top = top;
                data.bottom = bottom;
            }
        }
    }

    fn shuffle_molehills(&mut self, order: Vec<LevelOrder>) -> Result<(), Box<dyn Error>> {
        let mut molehills = vec![];
        for level in &order {
            for mole in LEVELS_INFO[*level].molehills {
                molehills.push(mole);
            }
        }

        let l0 = order[0].molehill_count(); // MM
        let l1 = order[1].molehill_count(); // TTC
        let l2 = order[2].molehill_count(); // CC
        let l3 = order[3].molehill_count(); // BGS

        let talon_trot_max_pos = l0;
        let beak_buster_max_pos = l0 + l1;
        let shock_jump_max_pos = l0 + l1 + l2 + l3;

        loop {
            molehills.shuffle(&mut rng());

            // since the first world needs talon trot
            if molehills[0..talon_trot_max_pos]
                .iter()
                .any(|m| m.ability == Ability::TalonTrot)
                // and beak buster is needed to access world 3
                && molehills[0..beak_buster_max_pos]
                    .iter()
                    .any(|m| m.ability == Ability::BeakBuster)
                // and shock jump is needed to access worlds 5+
                && molehills[0..shock_jump_max_pos]
                    .iter()
                    .any(|m| m.ability == Ability::ShockJump)
            {
                break;
            }
        }

        let mut xex = OpenOptions::new()
            .read(true)
            .write(true)
            .open("default.xex")
            .expect("Can't open default.xex, missing?");

        let mut mole_index = 0;
        for level in &order {
            for mole in LEVELS_INFO[*level].molehills {
                xex.seek(SeekFrom::Start(
                    MOLEHILLS_MOVES_DATA + mole.table_index as u64 * 6, /* or 8? */
                ))?;

                xex.write_u16::<BigEndian>(molehills[mole_index].teach_text_id.into())?;
                xex.write_u16::<BigEndian>(molehills[mole_index].refresher_text_id.into())?;
                xex.read_u8()?; // shorter than "skip"
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

        let mut xex = OpenOptions::new()
            .write(true)
            .open("default.xex")
            .expect("Can't open default.xex, missing?");
        let new_order: usize = new.into();
        xex.seek(SeekFrom::Start(LAIR_WARPS_TARGET + new_order as u64 * 4))?;
        xex.write_u16::<BigEndian>(old_level.warp_lair.map_id.into())?;
        xex.write_u16::<BigEndian>(old_level.warp_lair.exit_id)?;

        Ok(())
    }

    pub fn shuffle_entities(&mut self, actors: &Vec<ActorId>, sprites: &Vec<SpritePropId>) {
        println!("shuffle entities");

        for level in &LEVELS_INFO {
            self.shuffle_entities_for_level(actors, sprites, level);
        }
    }

    pub fn randomize_enemies(&mut self) {
        for level in &LEVELS_INFO {
            self.randomize_enemies_for_level(level);
        }

        for map in &LAIR_MAPS {
            self.randomize_enemies_for_map(map);
        }
    }

    fn randomize_enemies_for_level(&mut self, level: &LevelInfo) {
        for map_id in level.maps {
            self.randomize_enemies_for_map(map_id);
        }
    }

    fn randomize_enemies_for_map(&mut self, map_id: &MapSetupId) {
        let map = self.get_map_setup(*map_id);

        for cube in map.cubes.iter_mut() {
            for prop in cube.props_1.iter_mut() {
                if let Category::Actor(actor_id) = prop.category {
                    if actor_id.is_enemy() {
                        prop.category = Category::Actor(ActorId::random_enemy());
                    }
                }
            }
        }
    }

    fn shuffle_entities_for_level(
        &mut self,
        actors: &Vec<ActorId>,
        sprites: &Vec<SpritePropId>,
        level: &LevelInfo,
    ) {
        // it's the lair
        if level.maps.len() == 0 {
            return;
        }

        let mut grabbed_entities = vec![];

        for map in level.maps {
            let mut grabbed = self.grab_entities_from_map(&actors, &sprites, map);
            grabbed_entities.append(&mut grabbed);
        }

        let rng = &mut rng();
        for _ in 0..(grabbed_entities.len() * 4) {
            let a = rng.random_range(..grabbed_entities.len());
            let b = rng.random_range(..grabbed_entities.len());
            if a != b {
                let prop = grabbed_entities[a].prop;
                grabbed_entities[a].prop = grabbed_entities[b].prop;
                grabbed_entities[b].prop = prop;
                let linked = grabbed_entities[a].linked;
                grabbed_entities[a].linked = grabbed_entities[b].linked;
                grabbed_entities[b].linked = linked;
            }
        }

        for map in level.maps {
            self.insert_grabbed_entities_from_map(&grabbed_entities, map);
        }
    }

    fn insert_grabbed_entities_from_map(&mut self, entities: &Vec<Location>, map_id: &MapSetupId) {
        let id: u16 = (*map_id).into();
        let map = &mut self.assets[id as usize].asset;

        let Asset::MapSetup(map) = map else {
            unreachable!();
        };

        for entity in entities {
            if entity.map_id == *map_id {
                match entity.prop {
                    Props::Prop1(prop1) => {
                        map.cubes[entity.cube_id].props_1.push(Prop1 {
                            position: entity.position,
                            selector_or_radius: prop1.selector_or_radius,
                            category: prop1.category,
                            unk_bit_0: prop1.unk_bit_0,
                            marker_id: prop1.marker_id,
                            byte_0b: prop1.byte_0b,
                            bitfield_0c: prop1.bitfield_0c,
                            bitfield_10: prop1.bitfield_10,
                        });

                        if let Some(mut linked) = entity.linked {
                            linked.position = entity.position;
                            map.cubes[entity.cube_id].props_1.push(linked);
                        }
                    }
                    Props::Prop2(prop2) => {
                        let Prop2::Sprite {
                            id,
                            flags,
                            position: _,
                            bitfield_0a,
                        } = prop2
                        else {
                            unreachable!();
                        };

                        map.cubes[entity.cube_id].props_2.push(Prop2::Sprite {
                            id: id,
                            flags: flags,
                            position: entity.position,
                            bitfield_0a: bitfield_0a,
                        });
                    }
                }
            }
        }
    }

    fn grab_entities_from_map(
        &mut self,
        actors: &Vec<ActorId>,
        sprites: &Vec<SpritePropId>,
        map_id: &MapSetupId,
    ) -> Vec<Location> {
        let map = self.get_map_setup(*map_id);

        let mut locations = vec![];

        let mut saved_flags = vec![];
        for cube in map.cubes.iter_mut() {
            for prop in &cube.props_1 {
                if let Category::Flags(_) = prop.category {
                    saved_flags.push(*prop);
                }
            }

            cube.props_1.retain(|p| {
                if let Category::Flags(_) = p.category {
                    false
                } else {
                    true
                }
            });
        }

        for (cube_id, cube) in map.cubes.iter_mut().enumerate() {
            let mut saved_props = vec![];

            for prop in &cube.props_1 {
                if let Category::Actor(actor_id) = prop.category {
                    if actors.contains(&actor_id) {
                        let linked = if actor_id.needs_flag() {
                            let f = find_closest_flag(&prop.position, &mut saved_flags);
                            Some(f)
                        } else {
                            None
                        };

                        locations.push(Location {
                            map_id: *map_id,
                            cube_id,
                            position: prop.position,
                            prop: Props::Prop1(prop.clone()),
                            linked,
                        });
                    } else {
                        saved_props.push(prop.clone());
                    }
                } else {
                    saved_props.push(prop.clone());
                }
            }

            cube.props_1 = saved_props;

            let mut saved_props = vec![];

            for prop in &cube.props_2 {
                if let Prop2::Sprite {
                    id,
                    flags: _,
                    position,
                    bitfield_0a: _,
                } = *prop
                {
                    if sprites.contains(&id) {
                        locations.push(Location {
                            map_id: *map_id,
                            cube_id,
                            position,
                            prop: Props::Prop2(*prop),
                            linked: None,
                        });
                    } else {
                        saved_props.push(*prop);
                    }
                } else {
                    saved_props.push(*prop);
                }
            }

            cube.props_2 = saved_props;
        }

        if !saved_flags.is_empty() {
            for flag in saved_flags {
                let mut inserted = false;
                for cube in &mut map.cubes {
                    if compare_position(flag.position.x, cube.x)
                        && compare_position(flag.position.y, cube.y)
                        && compare_position(flag.position.z, cube.z)
                    {
                        cube.props_1.push(flag);
                        inserted = true;
                        break;
                    }
                }

                if !inserted {
                    unreachable!();
                }
            }
        }

        locations
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
            .open("db360.cmp")
            .expect("Can't write db360.cmp");

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
        let mut file = OpenOptions::new()
            .write(true)
            .open("db360.textures.cmp")
            .expect("Can't write db360.textures.cmp");
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
    let mut file = File::open("db360.cmp").expect("Can't open db360.cmp, missing?");

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
    let mut file =
        File::open("db360.textures.cmp").expect("Can't open db360.textures.cmp, missing?");
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

fn find_closest_flag(position: &Vector3<i16>, flags: &mut Vec<Prop1>) -> Prop1 {
    let mut closest = (99999999, None);
    for (idx, f) in flags.iter().enumerate() {
        let dist = distance(position, &f.position);

        if dist < closest.0 {
            closest.0 = dist;
            closest.1 = Some(idx);
        }
    }

    if closest.1.is_none() {
        panic!("{}", closest.0);
    }

    assert!(closest.0.isqrt() < 150);
    flags.remove(closest.1.unwrap())
}

fn distance(a: &Vector3<i16>, b: &Vector3<i16>) -> u32 {
    let x = (a.x - b.x).abs() as u32;
    let y = 0; //(a.y - b.y).abs() as u32;
    let z = (a.z - b.z).abs() as u32;

    x * x + y * y + z * z
}

fn get_door_flag(cost: u32) -> FileProgress {
    match cost {
        50 => FileProgress::NoteDoor50Open,
        180 => FileProgress::NoteDoor180Open,
        260 => FileProgress::NoteDoor260Open,
        350 => FileProgress::NoteDoor350Open,
        450 => FileProgress::NoteDoor450Open,
        640 => FileProgress::NoteDoor640Open,
        765 => FileProgress::NoteDoor765Open,
        810 => FileProgress::NoteDoor810Open,
        828 => FileProgress::NoteDoor828Open,
        846 => FileProgress::NoteDoor846Open,
        864 => FileProgress::NoteDoor864Open,
        882 => FileProgress::NoteDoor882Open,
        _ => unreachable!(),
    }
}

fn compare_position(position: i16, cube: i32) -> bool {
    let position = position as i32;
    let cube = cube * 1000;

    cube <= position && position < (cube + 1000)
}
