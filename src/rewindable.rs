use crate::{RewindCurveType, Vector3, rewind_curve::IRewindCurve};

#[derive(Debug)]
pub struct RewindableData {
    pub index: i32,
    pub text: String,
    pub rewind_type: RewindCurveType,
    pub curve: Box<dyn IRewindCurve>,
}

#[derive(Debug)]
pub struct Rewindable {
    pub go_name: String,
    pub type_name: String,
    pub ref_pos: Vector3,
}
