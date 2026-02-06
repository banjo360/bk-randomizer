use crate::assets::map_setup::{Prop1, Prop2};

#[derive(Copy, Clone)]
pub enum Props {
    Prop1(Prop1),
    Prop2(Prop2),
}
