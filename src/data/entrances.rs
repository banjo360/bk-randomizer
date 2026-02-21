use crate::enums::MapSetupId::*;
use crate::enums::{Ability, MapSetupId};

#[derive(Debug, Copy, Clone)]
pub enum Requirement {
    Ability(Ability),
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
                        target: GlFpLobby,
                        required: &[Requirement::Ability(Ability::ShockJump)],
                    },
                ],
            },
            Entrance {
                from: GlBgsLobby,
                paths: &[
                    Destination::new(Gl180NoteDoor),
                    Destination {
                        target: GlFpLobby,
                        required: &[Requirement::Ability(Ability::ShockJump)],
                    },
                ],
            },
            Entrance {
                from: GlFpLobby,
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
];
