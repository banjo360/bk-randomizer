use crate::enum_builder;

enum_builder! {
    #[repr(u16)]
    pub enum ActorId {
        BigButt = 4,
        Ticker = 5,
        Grublin = 6,
        Conga = 8,
        BeeHive = 18,
        Leaky = 30,
        Jiggy = 70,
        HoneyComb = 71,
        ExtraLife = 73,
        NoteDoor = 515,
    }
}

enum_builder! {
    #[repr(u16)]
    pub enum LevelSetup {
        SpiralMountain = 1821,
        MumbosMountain = 1822,
    }
}

enum_builder! {
    #[repr(u16)]
    pub enum Dialogue {
        CaptnBlubber = 2316,
    }
}

pub enum AssetId {
    LevelSetup(LevelSetup),
    Dialogue(Dialogue),
}

enum_builder! {
    #[repr(u8)]
    #[derive(Eq, PartialEq, Hash)]
    pub enum Language {
        English = 0,
        Japanese = 1,
        French = 2,
        German = 3,
    }
}
