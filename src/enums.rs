use crate::enum_builder;

enum_builder! {
    #[repr(u16)]
    pub enum ActorId {
        HoneyComb = 71,
        ExtraLife = 73,
    }
}
