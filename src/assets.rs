use animation::Animation;
use dialogue::Dialogue;
use map_setup::MapSetup;
use midi::Midi;
use model::Model;
use question::Question;
use sprite::Sprite;
use unknown::Unknown;

pub mod animation;
pub mod dialogue;
pub mod map_setup;
pub mod midi;
pub mod model;
pub mod question;
pub mod sprite;
pub mod unknown;

pub enum Asset {
    Animation(Animation),
    Dialogue(Dialogue),
    MapSetup(MapSetup),
    Question(Question),
    Unknown(Unknown),
    Sprite(Sprite),
    Model(Model),
    Midi(Midi),
    Empty,
}
