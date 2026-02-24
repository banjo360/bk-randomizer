use crate::enums::WarpOrTriggerId::*;
use crate::enums::file_progress::FileProgress;
use crate::enums::{Ability, MapSetupId};
use crate::enums::{MapSetupId::*, WarpOrTriggerId};

#[derive(Debug, Copy, Clone)]
pub enum Transformation {
    Termite,
    Pumpkin,
    Walrus,
    Croc,
    Bee,
}

#[derive(Debug, Copy, Clone)]
pub enum Requirement {
    Ability(Ability),
    Transformation(Transformation),
    Flag(FileProgress),
}

#[derive(Debug, Copy, Clone)]
pub struct Destination {
    pub target: MapSetupId,
    pub required: &'static [Requirement],
}

impl Destination {
    const fn new(target: MapSetupId) -> Self {
        Self {
            target,
            required: &[],
        }
    }
}

pub struct Entrance {
    pub from: MapSetupId,
    pub warp: WarpOrTriggerId,
    pub paths: &'static [Destination],
}

pub struct Map {
    pub id: MapSetupId,
    pub entrances: &'static [Entrance],
}

pub const MAPS: &'static [Map] = &[
    Map {
        id: SpiralMountain,
        entrances: &[
            Entrance {
                from: BanjosHouse,
                warp: WarpSmEnterBanjosHouse,
                paths: &[Destination::new(GlMmLobby)],
            },
            Entrance {
                from: GlMmLobby,
                warp: WarpLairEnterMmLobbyFromSmLevel,
                paths: &[Destination::new(BanjosHouse)],
            },
        ],
    },
    Map {
        id: BanjosHouse,
        entrances: &[Entrance {
            from: SpiralMountain,
            warp: WarpSmExitBanjosHouse,
            paths: &[],
        }],
    },
    Map {
        id: GlMmLobby,
        entrances: &[
            Entrance {
                from: SpiralMountain,
                warp: WarpSmExitLair,
                paths: &[
                    Destination::new(MumbosMountain),
                    Destination {
                        target: GlTtcAndCcPuzzle,
                        required: &[Requirement::Ability(Ability::TalonTrot)],
                    },
                ],
            },
            Entrance {
                from: MumbosMountain,
                warp: WarpMmEnterLevel,
                paths: &[
                    Destination::new(SpiralMountain),
                    Destination {
                        target: GlTtcAndCcPuzzle,
                        required: &[Requirement::Ability(Ability::TalonTrot)],
                    },
                ],
            },
            Entrance {
                from: GlTtcAndCcPuzzle,
                warp: WarpLairEnterPuzzlesRoomFromMmLobby,
                paths: &[
                    Destination::new(MumbosMountain),
                    Destination::new(SpiralMountain),
                ],
            },
        ],
    },
    Map {
        id: GlTtcAndCcPuzzle,
        entrances: &[
            Entrance {
                from: GlMmLobby,
                warp: WarpLairEnterMmLobbyFromPuzzlesRoom,
                paths: &[Destination::new(Gl180NoteDoor)],
            },
            Entrance {
                from: Gl180NoteDoor,
                warp: WarpLairEnterCcwPuzzleRoomFromPuzzlesRoom,
                paths: &[Destination::new(GlMmLobby)],
            },
        ],
    },
    Map {
        id: Gl180NoteDoor,
        entrances: &[
            Entrance {
                from: GlTtcAndCcPuzzle,
                warp: WarpLairEnterPuzzlesRoomFromCcwPuzzleRoom,
                paths: &[
                    Destination::new(GlCcLobby),
                    Destination::new(GlTtcLobby),
                    Destination {
                        target: GlStatueRoom,
                        required: &[Requirement::Ability(Ability::TalonTrot)],
                    },
                ],
            },
            Entrance {
                from: GlCcLobby,
                warp: WarpLairEnterCCLobbyFromCcwPuzzleRoom,
                paths: &[
                    Destination::new(GlTtcAndCcPuzzle),
                    Destination::new(GlTtcLobby),
                    Destination {
                        target: GlStatueRoom,
                        required: &[Requirement::Ability(Ability::TalonTrot)],
                    },
                ],
            },
            Entrance {
                from: GlTtcLobby,
                warp: WarpLairEnterTtcLobbyFromCcwPuzzleRoom,
                paths: &[
                    Destination::new(GlTtcAndCcPuzzle),
                    Destination::new(GlCcLobby),
                    Destination {
                        target: GlStatueRoom,
                        required: &[Requirement::Ability(Ability::TalonTrot)],
                    },
                ],
            },
            Entrance {
                from: GlStatueRoom,
                warp: WarpLairEnterPointingGruntyStatueFromCcwPuzzleRoom,
                paths: &[
                    Destination::new(GlTtcAndCcPuzzle),
                    Destination::new(GlCcLobby),
                    Destination::new(GlTtcLobby),
                ],
            },
        ],
    },
    Map {
        id: GlTtcLobby,
        entrances: &[
            Entrance {
                from: Gl180NoteDoor,
                warp: WarpLairEnterTtcLobbyFromCcwPuzzleRoom,
                paths: &[Destination::new(TreasureTroveCove)],
            },
            Entrance {
                from: TreasureTroveCove,
                warp: WarpTtcEnterLevel,
                paths: &[Destination::new(Gl180NoteDoor)],
            },
        ],
    },
    Map {
        id: GlCcLobby,
        entrances: &[
            Entrance {
                from: Gl180NoteDoor,
                warp: WarpLairEnterCcwPuzzleRoomFromCCLobby,
                paths: &[Destination {
                    target: ClankersCavern,
                    required: &[Requirement::Ability(Ability::BeakBuster)],
                }],
            },
            Entrance {
                from: ClankersCavern,
                warp: WarpCcEnterLevel,
                paths: &[Destination::new(Gl180NoteDoor)],
            },
        ],
    },
    Map {
        id: GlStatueRoom,
        entrances: &[
            Entrance {
                from: Gl180NoteDoor,
                warp: WarpLairEnterCcwPuzzleFromPointingGruntyStatueRoom,
                paths: &[
                    Destination {
                        target: GlBgsLobby,
                        required: &[Requirement::Ability(Ability::TalonTrot)],
                    },
                    Destination {
                        target: GlGvLobby,
                        required: &[Requirement::Ability(Ability::ShockJump)],
                    },
                ],
            },
            Entrance {
                from: GlBgsLobby,
                warp: WarpLairEnterBgsLobbyFromPointingGruntyStatueRoom,
                paths: &[
                    Destination::new(Gl180NoteDoor),
                    Destination {
                        target: GlGvLobby,
                        required: &[Requirement::Ability(Ability::ShockJump)],
                    },
                ],
            },
            Entrance {
                from: GlGvLobby,
                warp: WarpLairEnterGvLobbyFromPointingStatueRoom,
                paths: &[
                    Destination::new(Gl180NoteDoor),
                    Destination {
                        target: GlBgsLobby,
                        required: &[Requirement::Ability(Ability::TalonTrot)],
                    },
                ],
            },
        ],
    },
    Map {
        id: GlBgsLobby,
        entrances: &[
            Entrance {
                from: GlStatueRoom,
                warp: WarpLairEnterPointingGruntyStatueFromBgsLobby,
                paths: &[Destination::new(BubbleGloopSwamp)],
            },
            Entrance {
                from: BubbleGloopSwamp,
                warp: WarpBgsEnterLevel,
                paths: &[Destination::new(GlStatueRoom)],
            },
        ],
    },
    Map {
        id: GlGvLobby,
        entrances: &[
            Entrance {
                from: GlStatueRoom,
                warp: WarpLairEnterPointingStatueRoomFromGvLobbyNoteDoor,
                paths: &[Destination::new(GobisValley), Destination::new(GlFpLobby)],
            },
            Entrance {
                from: GobisValley,
                warp: WarpGvEnterLevel,
                paths: &[Destination::new(GlStatueRoom), Destination::new(GlFpLobby)],
            },
            Entrance {
                from: GlFpLobby,
                warp: WarpLairEnterFpLobbyFromGvLobby,
                paths: &[
                    Destination::new(GlStatueRoom),
                    Destination::new(GobisValley),
                ],
            },
        ],
    },
    Map {
        id: GlFpLobby,
        entrances: &[
            Entrance {
                from: GlGvLobby,
                warp: WarpLairEnterGvLobbyFromFpLobby,
                paths: &[
                    Destination::new(GlGvPuzzle),
                    Destination::new(Gl640NoteDoor),
                    Destination::new(FreezeezyPeak),
                ],
            },
            Entrance {
                from: GlGvPuzzle,
                warp: WarpLairEnterGvPuzzleRoomFromFpLobby,
                paths: &[
                    Destination::new(GlGvLobby),
                    Destination::new(Gl640NoteDoor),
                    Destination::new(FreezeezyPeak),
                ],
            },
            Entrance {
                from: Gl640NoteDoor,
                warp: WarpLairEnter640NoteDoorRoomFromFpLobby,
                paths: &[
                    Destination::new(GlGvLobby),
                    Destination::new(GlGvPuzzle),
                    Destination::new(FreezeezyPeak),
                ],
            },
            Entrance {
                from: FreezeezyPeak,
                warp: WarpFpEnterLevel,
                paths: &[
                    Destination::new(GlGvLobby),
                    Destination::new(GlGvPuzzle),
                    Destination::new(Gl640NoteDoor),
                ],
            },
        ],
    },
    Map {
        id: GlGvPuzzle,
        entrances: &[
            Entrance {
                from: GlFpLobby,
                warp: WarpLairFpLobbyFromGvPuzzleRoom,
                paths: &[Destination::new(GlMmmLobby)],
            },
            Entrance {
                from: GlMmmLobby,
                warp: WarpLairEnterMmmLobbyFromGvPuzzleRoom,
                paths: &[Destination::new(GlFpLobby)],
            },
        ],
    },
    Map {
        id: GlMmmLobby,
        entrances: &[
            Entrance {
                from: GlGvPuzzle,
                warp: WarpLairEnterGvPuzzleRoomFromMmmLobby,
                paths: &[
                    Destination {
                        target: GlCrypt,
                        required: &[Requirement::Transformation(Transformation::Pumpkin)],
                    },
                    Destination::new(MadMonsterMansion),
                ],
            },
            Entrance {
                from: GlCrypt,
                warp: WarpLairEnterCryptFromMmmLobby,
                paths: &[
                    Destination::new(GlGvPuzzle),
                    Destination::new(MadMonsterMansion),
                ],
            },
            Entrance {
                from: MadMonsterMansion,
                warp: WarpMmmEnterLevel,
                paths: &[
                    Destination {
                        target: GlCrypt,
                        required: &[Requirement::Transformation(Transformation::Pumpkin)],
                    },
                    Destination::new(GlGvPuzzle),
                ],
            },
        ],
    },
    Map {
        id: GlCrypt,
        entrances: &[Entrance {
            from: GlMmmLobby,
            warp: WarpLairEnterMmmLobbyFromCrypt,
            paths: &[],
        }],
    },
    Map {
        id: Gl640NoteDoor,
        entrances: &[
            Entrance {
                from: GlFpLobby,
                warp: WarpLairEnterFpLobbyFrom640NoteDoorRoom,
                paths: &[
                    Destination {
                        target: GlCcwLobby,
                        required: &[Requirement::Flag(FileProgress::WaterSwitch2Pressed)],
                    },
                    Destination::new(GlRbbLobby),
                ],
            },
            Entrance {
                from: GlCcwLobby,
                warp: WarpLairCcwLobbyFrom640NoteDoorRoomDoorEntrance,
                paths: &[Destination::new(GlFpLobby), Destination::new(GlRbbLobby)],
            },
            Entrance {
                from: GlRbbLobby,
                warp: WarpLairRbbLobbyFrom640NoteDoorRoom,
                paths: &[
                    Destination {
                        target: GlCcwLobby,
                        required: &[Requirement::Flag(FileProgress::WaterSwitch2Pressed)],
                    },
                    Destination::new(GlFpLobby),
                ],
            },
        ],
    },
    Map {
        id: GlRbbLobby,
        entrances: &[
            Entrance {
                from: Gl640NoteDoor,
                warp: WarpLairEnter640NoteDoorRoomFromRbbLobby,
                paths: &[
                    Destination {
                        target: RustyBucketBay,
                        required: &[Requirement::Flag(FileProgress::WaterSwitch1Pressed)],
                    },
                    Destination {
                        target: GlRbbAndMmmPuzzle,
                        required: &[Requirement::Flag(FileProgress::WaterSwitch2Pressed)],
                    },
                    Destination::new(GlRbbAndMmmPuzzle),
                ],
            },
            Entrance {
                from: GlRbbAndMmmPuzzle,
                warp: WarpLairEnterMmmPuzzleFromRbbLobby,
                paths: &[
                    Destination {
                        target: RustyBucketBay,
                        required: &[Requirement::Flag(FileProgress::WaterSwitch1Pressed)],
                    },
                    Destination::new(GlRbbAndMmmPuzzle),
                    Destination::new(Gl640NoteDoor),
                ],
            },
            Entrance {
                from: GlRbbAndMmmPuzzle,
                warp: WarpLairEnterRbbPuzzleFromRbbLobby,
                paths: &[
                    Destination {
                        target: RustyBucketBay,
                        required: &[Requirement::Flag(FileProgress::WaterSwitch1Pressed)],
                    },
                    Destination {
                        target: GlRbbAndMmmPuzzle,
                        required: &[Requirement::Flag(FileProgress::WaterSwitch2Pressed)],
                    },
                    Destination::new(Gl640NoteDoor),
                ],
            },
            Entrance {
                from: RustyBucketBay,
                warp: WarpRbbEnterLevel,
                paths: &[
                    Destination {
                        target: GlRbbAndMmmPuzzle,
                        required: &[Requirement::Flag(FileProgress::WaterSwitch2Pressed)],
                    },
                    Destination::new(GlRbbAndMmmPuzzle),
                    Destination::new(Gl640NoteDoor),
                ],
            },
        ],
    },
    Map {
        id: GlRbbAndMmmPuzzle,
        entrances: &[
            Entrance {
                from: GlRbbLobby,
                warp: WarpLairEnterRbbLobbyFromRbbPuzzleRoom,
                paths: &[Destination::new(GlRbbLobby)],
            },
            Entrance {
                from: GlRbbLobby,
                warp: WarpLairEnterRbbLobbyFromMmmPuzzleRoom,
                paths: &[Destination::new(GlRbbLobby)],
            },
        ],
    },
    Map {
        id: GlCcwLobby,
        entrances: &[
            Entrance {
                from: Gl640NoteDoor,
                warp: WarpLairEnter640NoteDoorRoomFromCcwLobbyDoorEntrance,
                paths: &[
                    Destination {
                        target: CcwHub,
                        required: &[Requirement::Ability(Ability::TalonTrot)],
                    },
                    Destination::new(GlFfEntrance),
                ],
            },
            Entrance {
                from: CcwHub,
                warp: WarpCcwEnterLevel,
                paths: &[
                    Destination::new(Gl640NoteDoor),
                    Destination::new(GlFfEntrance),
                ],
            },
            Entrance {
                from: GlFfEntrance,
                warp: WarpLairEnterFurnaceFunPathFromCcwLobby,
                paths: &[
                    Destination::new(Gl640NoteDoor),
                    Destination {
                        target: CcwHub,
                        required: &[Requirement::Ability(Ability::TalonTrot)],
                    },
                ],
            },
        ],
    },
    Map {
        id: GlFfEntrance,
        entrances: &[Entrance {
            from: GlCcwLobby,
            warp: WarpLairEnterCcwLobbyFromFurnaceFunPath,
            paths: &[],
        }],
    },
    Map {
        id: BubbleGloopSwamp,
        entrances: &[Entrance {
            from: GlBgsLobby,
            warp: WarpOrTriggerId::Unknown(0x8003),
            paths: &[],
        }],
    },
    Map {
        id: CcwHub,
        entrances: &[Entrance {
            from: GlCcwLobby,
            warp: WarpOrTriggerId::Unknown(0x8007),
            paths: &[],
        }],
    },
    Map {
        id: ClankersCavern,
        entrances: &[Entrance {
            from: GlCcLobby,
            warp: WarpOrTriggerId::Unknown(0x8002),
            paths: &[],
        }],
    },
    Map {
        id: FreezeezyPeak,
        entrances: &[Entrance {
            from: GlFpLobby,
            warp: WarpOrTriggerId::Unknown(0x8004),
            paths: &[],
        }],
    },
    Map {
        id: GobisValley,
        entrances: &[Entrance {
            from: GlGvLobby,
            warp: WarpOrTriggerId::Unknown(0x8006),
            paths: &[],
        }],
    },
    Map {
        id: MadMonsterMansion,
        entrances: &[Entrance {
            from: GlMmmLobby,
            warp: WarpOrTriggerId::Unknown(0x8009),
            paths: &[],
        }],
    },
    Map {
        id: MumbosMountain,
        entrances: &[Entrance {
            from: GlMmLobby,
            warp: WarpOrTriggerId::Unknown(0x8000),
            paths: &[],
        }],
    },
    Map {
        id: RustyBucketBay,
        entrances: &[Entrance {
            from: GlRbbLobby,
            warp: WarpOrTriggerId::Unknown(0x8008),
            paths: &[],
        }],
    },
    Map {
        id: TreasureTroveCove,
        entrances: &[Entrance {
            from: GlTtcLobby,
            warp: WarpOrTriggerId::Unknown(0x8002),
            paths: &[],
        }],
    },
];

pub fn get_warp_data(from: MapSetupId, to: MapSetupId) -> WarpOrTriggerId {
    for map in MAPS {
        if map.id == from {
            for entrance in map.entrances {
                if entrance.from == to {
                    return entrance.warp;
                }
            }
            break;
        }
    }

    panic!("There are no warps between {from} and {to}");
}
