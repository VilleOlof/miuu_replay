use crate::{quaternion::Quaternion, vector2::Vector2, vector3::Vector3};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
#[allow(unused)]
pub enum RewindCurveType {
    Unknown = -1,
    Float,
    Int,
    Bool,
    Vector2,
    Vector3,
    Quaternion,
    UShort,
    UInt32,
    UInt32Array,
    Int32Array,
}

impl RewindCurveType {
    pub fn from_i32(num: i32) -> Self {
        unsafe { std::mem::transmute(num) }
    }

    pub fn size(&self) -> usize {
        use std::mem::size_of;
        match self {
            Self::Float => size_of::<f32>(),
            Self::Int => size_of::<i32>(),
            Self::Bool => size_of::<bool>(),
            Self::Vector2 => size_of::<Vector2>(),
            Self::Vector3 => size_of::<Vector3>(),
            Self::Quaternion => size_of::<Quaternion>(),
            Self::UShort => size_of::<u16>(),
            Self::UInt32 => size_of::<u32>(),
            Self::UInt32Array => size_of::<u32>(),
            Self::Int32Array => size_of::<i32>(),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn get_curve_type_for_array_type(&self) -> Self {
        match self {
            Self::UInt32Array => Self::UInt32,
            Self::Int32Array => Self::Int,
            _ => Self::Unknown,
        }
    }
}
