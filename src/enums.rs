pub use self::actors::ActorId;
pub use self::animations::AnimationId;
pub use self::credits::CreditsId;
pub use self::dialogues::DialogueId;
pub use self::map_setups::MapSetupId;
pub use self::midis::MidiId;
pub use self::models::ModelId;
pub use self::questions::QuestionId;
pub use self::sprites::SpriteId;
pub use self::textures::TextureId;
pub use self::warp_or_trigger::WarpOrTriggerId;
pub use self::xbox::XboxId;
use crate::enum_builder;

enum_builder! {
    #[repr(u16)]
    pub enum Transform {
        XRotation    = 0,
        YRotation    = 1,
        ZRotation    = 2,
        XScale       = 3,
        YScale       = 4,
        ZScale       = 5,
        XTranslation = 6,
        YTranslation = 7,
        ZTranslation = 8,
    }
}

enum_builder! {
    #[repr(u16)]
    pub enum UnknownId {
        Unknown0064D898 = 1770,
        Unknown00702780 = 2212,
        Unknown00703D04 = 2213,
        Unknown00705390 = 2214,
        Unknown00706EFC = 2215,
        Unknown007080F6 = 2216,
        Unknown0070916E = 2217,
        Unknown00709B0E = 2218,
        Unknown0070B52A = 2219,
        Unknown0070D19E = 2220,
        Unknown0070E31A = 2221,
        Unknown0070F52C = 2222,
        Unknown00710814 = 2225,
        Unknown007172DC = 2227,
        Unknown00721C6C = 2228,
        Unknown0072C97C = 2229,
        Unknown0073260A = 2230,
        Unknown0073D31A = 2231,
        Unknown0074002A = 2232,
        Unknown00745CB8 = 2233,
        Unknown0074C7EC = 2234,
        Unknown00755684 = 2235,
        Unknown0075BD50 = 2242,
        Unknown0075CDE8 = 2248,
        Unknown0075E010 = 2252,
        Unknown00760888 = 2255,
        Unknown00762038 = 2257,
        Unknown00763230 = 2259,
        Unknown00763688 = 2260,
        Unknown00763AF8 = 2264,
        Unknown00764E30 = 2265,
        Unknown00766270 = 2266,
        Unknown00767EE0 = 2267,
        Unknown00769050 = 2268,
        Unknown0076A0C8 = 2269,
        Unknown0076AA68 = 2270,
        Unknown0076C2E0 = 2271,
        Unknown0076E090 = 2272,
        Unknown0076F120 = 2273,
        Unknown00770398 = 2274,
        Unknown00771680 = 2277,
        Unknown00778148 = 2279,
        Unknown00782AD8 = 2280,
        Unknown007885F8 = 2281,
        Unknown0078E520 = 2283,
        Unknown00791230 = 2284,
        Unknown00797A08 = 2285,
        Unknown0079D8A8 = 2286,
        Unknown007A3950 = 2287,
        Unknown007AB5F0 = 2294,
        Unknown007AC688 = 2300,
        Unknown007AD598 = 2304,
        Unknown007AF9F0 = 2307,
        Unknown007B0EC8 = 2309,
        Unknown007B1DC0 = 2311,
        Unknown007B2218 = 2312,
        Unknown011E5751 = 3642,
        Unknown011E583D = 3643,
        Unknown011EA54B = 3700,
    }
}

enum_builder! {
    #[repr(u8)]
    pub enum Ability {
        Barge = 0x0,
        BeakBomb = 0x1,
        BeakBuster = 0x2,
        CameraControl = 0x3,
        ClawSwipe = 0x4,
        Climb = 0x5,
        Eggs = 0x6,
        FeatheryFlap = 0x7,
        FlapFlip = 0x8,
        Flight = 0x9,
        HoldAJumpHigher = 0xA,
        RatatatRap = 0xB,
        Roll = 0xC,
        ShockJump = 0xD,
        WadingBoots = 0xE,
        Dive = 0xF,
        TalonTrot = 0x10,
        TurboTalon = 0x11,
        Wonderwing = 0x12,
        FirstNotedoor = 0x13,
    }
}

enum_builder! {
    #[repr(u16)]
    pub enum SpritePropId {
        RedFeather = 14,
        BlueTulip = 70,
        OrangeDaisy = 71,
        RedDaisy = 80,
        ConchShell = 289,
        GoldFeather = 351,
        ThinSeaweed = 354,
        ThickSeaweed = 355,
        MusicalNote = 356,
        BlueEgg = 357,
    }
}

pub mod actors;
pub mod animations;
pub mod credits;
pub mod dialogues;
pub mod map;
pub mod map_setups;
pub mod midis;
pub mod models;
pub mod questions;
pub mod sprites;
pub mod textures;
pub mod warp_or_trigger;
pub mod xbox;

pub enum AssetId {
    Animation(AnimationId),
    Credits(CreditsId),
    Dialogue(DialogueId),
    Empty,
    MapSetup(MapSetupId),
    Midi(MidiId),
    Model(ModelId),
    Question(QuestionId),
    Sprite(SpriteId),
    Unknown(UnknownId),
    Xbox(XboxId),
}

impl std::fmt::Display for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetId::Animation(animation_id) => write!(f, "Animation({animation_id})"),
            AssetId::Credits(credits_id) => write!(f, "Credits({credits_id})"),
            AssetId::Dialogue(dialogue_id) => write!(f, "Dialogue({dialogue_id})"),
            AssetId::Empty => write!(f, "Empty"),
            AssetId::MapSetup(map_setup_id) => write!(f, "MapSetup({map_setup_id})"),
            AssetId::Midi(midi_id) => write!(f, "Midi({midi_id})"),
            AssetId::Model(model_id) => write!(f, "Model({model_id})"),
            AssetId::Question(question_id) => write!(f, "Question({question_id})"),
            AssetId::Sprite(sprite_id) => write!(f, "Sprite({sprite_id})"),
            AssetId::Unknown(unknown_id) => write!(f, "Unknown({unknown_id})"),
            AssetId::Xbox(xbox_id) => write!(f, "Xbox({xbox_id})"),
        }
    }
}

enum_builder! {
    #[repr(u8)]
    #[derive(Eq, Hash)]
    pub enum Language {
        English = 0,
        Japanese = 1,
        French = 2,
        German = 3,
    }
}
