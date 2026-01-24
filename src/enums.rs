#[repr(u16)]
#[derive(Copy, Clone)]
pub enum ActorId {
    Unknown(u16),
    ExtraLife = 71,
    HoneyComb = 73,
}

impl Into<u16> for ActorId {
    fn into(self) -> u16 {
        match self {
            ActorId::ExtraLife => 71,
            ActorId::HoneyComb => 73,
            ActorId::Unknown(value) => value,
        }
    }
}

impl From<u16> for ActorId {
    fn from(value: u16) -> Self {
        match value {
            71 => ActorId::ExtraLife,
            73 => ActorId::HoneyComb,
            _ => ActorId::Unknown(value),
        }
    }
}
