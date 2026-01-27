use crate::ActorId;
use crate::MapSetupId;

#[repr(usize)]
enum LevelId {
    MumbosMountain = 0,
    TreasureTroveCove,
    ClankersCavern,
    BubblegloopSwamp,
    FreezeezyPeak,
    Lair,
    GobisValley,
    ClickClockWood,
    RustyBucketBay,
    MadMonsterMansion,
}

const MAIN_MAP_SETUP_ID: [MapSetupId; 10] = [
    MapSetupId::MumbosMountain,
    MapSetupId::TreasureTroveCove,
    MapSetupId::ClankersCavern,
    MapSetupId::BubbleGloopSwamp,
    MapSetupId::FreezeezyPeak,
    MapSetupId::TestHouse,
    MapSetupId::GobisValley,
    MapSetupId::CcwHub,
    MapSetupId::RustyBucketBay,
    MapSetupId::MadMonsterMansion,
];

const ENTRY_POINTS: [ActorId; 10] = [
    ActorId::EntryPoint5,
    ActorId::EntryPoint4,
    ActorId::EntryPoint5,
    ActorId::EntryPoint2,
    ActorId::EntryPoint1,
    ActorId::Bigbutt, // the lair, unused
    ActorId::EntryPoint8,
    ActorId::EntryPoint7,
    ActorId::EntryPoint16,
    ActorId::EntryPoint20,
];
