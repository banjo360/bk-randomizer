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
    pub enum MapSetupId {
        SpiralMountain = 0x71D,
        MumbosMountain = 0x71E,
        BlubbersShip = 0x721,
        NippersShell = 0x722,
        TreasureTroveCove = 0x723,
        Sandcastle = 0x726,
        ClankersCavern = 0x727,
        TermiteHill = 0x728,
        BubbleGloopSwamp = 0x729,
        MumbosSkull = 0x72A,
        MrVile = 0x72C,
        TipTupChoir = 0x72D,
    }
}

enum_builder! {
    #[repr(u16)]
    pub enum DialogueId {
        CaptnBlubber = 0x90C,
    }
}

pub enum AssetId {
    MapSetup(MapSetupId),
    Dialogue(DialogueId),
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
