use csharp_binary_encoding::BinaryReader;

use crate::{
    ReplayError, RewindCurveFitter, RewindCurveFitterArray, quaternion::Quaternion,
    rewind_curve::IRewindCurve, vector2::Vector2, vector3::Vector3,
};

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
    /// Creates a [`RewindCurveType`] from a [`i32`]
    pub unsafe fn from_i32(num: i32) -> Self {
        unsafe { std::mem::transmute(num) }
    }

    /// Returns the matching byte size of the type that the [`RewindCurveType`] corresponds to.  
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

    /// Returns the matching item type for an array type.  
    pub(crate) fn get_curve_type_for_array_type(&self) -> Self {
        match self {
            Self::UInt32Array => Self::UInt32,
            Self::Int32Array => Self::Int,
            _ => Self::Unknown,
        }
    }

    pub(crate) fn read_from(
        &self,
        reader: &mut BinaryReader<&[u8]>,
    ) -> Result<IRewindCurve, ReplayError> {
        Ok(match self {
            RewindCurveType::Unknown => unimplemented!(),
            RewindCurveType::Float => {
                IRewindCurve::Float(RewindCurveFitter::<f32>::read_from(reader, *self)?)
            }
            RewindCurveType::Int => {
                IRewindCurve::Int(RewindCurveFitter::<i32>::read_from(reader, *self)?)
            }
            RewindCurveType::Bool => {
                IRewindCurve::Bool(RewindCurveFitter::<bool>::read_from(reader, *self)?)
            }
            RewindCurveType::Vector2 => {
                IRewindCurve::Vector2(RewindCurveFitter::<Vector2>::read_from(reader, *self)?)
            }
            RewindCurveType::Vector3 => {
                IRewindCurve::Vector3(RewindCurveFitter::<Vector3>::read_from(reader, *self)?)
            }
            RewindCurveType::Quaternion => {
                IRewindCurve::Quaternion(RewindCurveFitter::<Quaternion>::read_from(reader, *self)?)
            }
            RewindCurveType::UShort => {
                IRewindCurve::UShort(RewindCurveFitter::<u16>::read_from(reader, *self)?)
            }
            RewindCurveType::UInt32 => {
                IRewindCurve::UInt32(RewindCurveFitter::<u32>::read_from(reader, *self)?)
            }
            RewindCurveType::UInt32Array => {
                IRewindCurve::UInt32Array(RewindCurveFitterArray::<u32>::read_from(reader, *self)?)
            }
            RewindCurveType::Int32Array => {
                IRewindCurve::Int32Array(RewindCurveFitterArray::<i32>::read_from(reader, *self)?)
            }
        })
    }
}
