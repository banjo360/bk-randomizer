use super::enums::Props;
use crate::enums::MapSetupId;
use crate::utils::Vector3;

pub struct Location {
    pub map_id: MapSetupId,
    pub cube_id: usize,
    pub position: Vector3<i16>,
    pub prop: Props,
}
