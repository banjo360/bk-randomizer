use crate::ActorId;
use crate::MapSetupId;

struct LevelInfo {
    setup_id: MapSetupId,
    warp_entry_point: ActorId,
}

const LEVELS_INFO: [LevelInfo; 9] = [
    LevelInfo {
        setup_id: MapSetupId::MumbosMountain,
        warp_entry_point: ActorId::EntryPoint5,
    },
    LevelInfo {
        setup_id: MapSetupId::TreasureTroveCove,
        warp_entry_point: ActorId::EntryPoint4,
    },
    LevelInfo {
        setup_id: MapSetupId::ClankersCavern,
        warp_entry_point: ActorId::EntryPoint5,
    },
    LevelInfo {
        setup_id: MapSetupId::BubbleGloopSwamp,
        warp_entry_point: ActorId::EntryPoint2,
    },
    LevelInfo {
        setup_id: MapSetupId::FreezeezyPeak,
        warp_entry_point: ActorId::EntryPoint1,
    },
    LevelInfo {
        setup_id: MapSetupId::GobisValley,
        warp_entry_point: ActorId::EntryPoint8,
    },
    LevelInfo {
        setup_id: MapSetupId::CcwHub,
        warp_entry_point: ActorId::EntryPoint7,
    },
    LevelInfo {
        setup_id: MapSetupId::RustyBucketBay,
        warp_entry_point: ActorId::EntryPoint16,
    },
    LevelInfo {
        setup_id: MapSetupId::MadMonsterMansion,
        warp_entry_point: ActorId::EntryPoint20,
    },
];
