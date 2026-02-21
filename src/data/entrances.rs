use crate::enums::MapSetupId::*;
use crate::enums::file_progress::FileProgress;
use crate::enums::{Ability, MapSetupId};

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
                paths: &[Destination::new(GlMmLobby)],
            },
            Entrance {
                from: GlMmLobby,
                paths: &[Destination::new(BanjosHouse)],
            },
        ],
    },
    Map {
        id: BanjosHouse,
        entrances: &[Entrance {
            from: SpiralMountain,
            paths: &[],
        }],
    },
    Map {
        id: GlMmLobby,
        entrances: &[
            Entrance {
                from: SpiralMountain,
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
                paths: &[Destination::new(Gl180NoteDoor)],
            },
            Entrance {
                from: Gl180NoteDoor,
                paths: &[Destination::new(GlMmLobby)],
            },
        ],
    },
    Map {
        id: Gl180NoteDoor,
        entrances: &[
            Entrance {
                from: GlTtcAndCcPuzzle,
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
                paths: &[Destination::new(TreasureTroveCove)],
            },
            Entrance {
                from: TreasureTroveCove,
                paths: &[Destination::new(Gl180NoteDoor)],
            },
        ],
    },
    Map {
        id: GlCcLobby,
        entrances: &[
            Entrance {
                from: Gl180NoteDoor,
                paths: &[Destination {
                    target: ClankersCavern,
                    required: &[Requirement::Ability(Ability::BeakBuster)],
                }],
            },
            Entrance {
                from: ClankersCavern,
                paths: &[Destination::new(Gl180NoteDoor)],
            },
        ],
    },
    Map {
        id: GlStatueRoom,
        entrances: &[
            Entrance {
                from: Gl180NoteDoor,
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
                paths: &[Destination::new(BubbleGloopSwamp)],
            },
            Entrance {
                from: BubbleGloopSwamp,
                paths: &[Destination::new(GlStatueRoom)],
            },
        ],
    },
    Map {
        id: GlGvLobby,
        entrances: &[
            Entrance {
                from: GlStatueRoom,
                paths: &[Destination::new(GobisValley), Destination::new(GlFpLobby)],
            },
            Entrance {
                from: GobisValley,
                paths: &[Destination::new(GlStatueRoom), Destination::new(GlFpLobby)],
            },
            Entrance {
                from: GlFpLobby,
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
                paths: &[
                    Destination::new(GlGvPuzzle),
                    Destination::new(Gl640NoteDoor),
                    Destination::new(FreezeezyPeak),
                ],
            },
            Entrance {
                from: GlGvPuzzle,
                paths: &[
                    Destination::new(GlGvLobby),
                    Destination::new(Gl640NoteDoor),
                    Destination::new(FreezeezyPeak),
                ],
            },
            Entrance {
                from: Gl640NoteDoor,
                paths: &[
                    Destination::new(GlGvLobby),
                    Destination::new(GlGvPuzzle),
                    Destination::new(FreezeezyPeak),
                ],
            },
            Entrance {
                from: FreezeezyPeak,
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
                paths: &[Destination::new(GlMmmLobby)],
            },
            Entrance {
                from: GlMmmLobby,
                paths: &[Destination::new(GlFpLobby)],
            },
        ],
    },
    Map {
        id: GlMmmLobby,
        entrances: &[
            Entrance {
                from: GlGvPuzzle,
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
                paths: &[
                    Destination::new(GlGvPuzzle),
                    Destination::new(MadMonsterMansion),
                ],
            },
            Entrance {
                from: MadMonsterMansion,
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
            paths: &[],
        }],
    },
    Map {
        id: Gl640NoteDoor,
        entrances: &[
            Entrance {
                from: GlFpLobby,
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
                paths: &[Destination::new(GlFpLobby), Destination::new(GlRbbLobby)],
            },
            Entrance {
                from: GlRbbLobby,
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
                paths: &[Destination::new(GlRbbLobby)],
            },
            Entrance {
                from: GlRbbLobby,
                paths: &[Destination::new(GlRbbLobby)],
            },
        ],
    },
    Map {
        id: GlCcwLobby,
        entrances: &[
            Entrance {
                from: Gl640NoteDoor,
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
                paths: &[
                    Destination::new(Gl640NoteDoor),
                    Destination::new(GlFfEntrance),
                ],
            },
            Entrance {
                from: GlFfEntrance,
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
            paths: &[],
        }],
    },
    Map {
        id: BubbleGloopSwamp,
        entrances: &[Entrance {
            from: GlBgsLobby,
            paths: &[],
        }],
    },
    Map {
        id: CcwHub,
        entrances: &[Entrance {
            from: GlCcwLobby,
            paths: &[],
        }],
    },
    Map {
        id: ClankersCavern,
        entrances: &[Entrance {
            from: GlCcLobby,
            paths: &[],
        }],
    },
    Map {
        id: FreezeezyPeak,
        entrances: &[Entrance {
            from: GlFpLobby,
            paths: &[],
        }],
    },
    Map {
        id: GobisValley,
        entrances: &[Entrance {
            from: GlGvLobby,
            paths: &[],
        }],
    },
    Map {
        id: MadMonsterMansion,
        entrances: &[Entrance {
            from: GlMmmLobby,
            paths: &[],
        }],
    },
    Map {
        id: MumbosMountain,
        entrances: &[Entrance {
            from: GlMmLobby,
            paths: &[],
        }],
    },
    Map {
        id: RustyBucketBay,
        entrances: &[Entrance {
            from: GlRbbLobby,
            paths: &[],
        }],
    },
    Map {
        id: TreasureTroveCove,
        entrances: &[Entrance {
            from: GlTtcLobby,
            paths: &[],
        }],
    },
];
