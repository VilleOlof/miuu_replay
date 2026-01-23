use std::fmt::Debug;

use csharp_binary_encoding::BinaryReader;

use crate::{
    Quaternion, ReplayError, RewindCurveFitter, RewindCurveFitterArray, Vector2, Vector3,
    circular_buffer::CircularBuffer, rewind_curve_type::RewindCurveType,
};

#[derive(Debug, Clone)]
pub enum IRewindCurve {
    Float(RewindCurveFitter<f32>),
    Int(RewindCurveFitter<i32>),
    Bool(RewindCurveFitter<bool>),
    Vector2(RewindCurveFitter<Vector2>),
    Vector3(RewindCurveFitter<Vector3>),
    Quaternion(RewindCurveFitter<Quaternion>),
    UShort(RewindCurveFitter<u16>),
    UInt32(RewindCurveFitter<u32>),
    UInt32Array(RewindCurveFitterArray<u32>),
    Int32Array(RewindCurveFitterArray<i32>),
}

#[derive(Debug, Clone)]
pub struct RewindCurve<T: Clone> {
    pub interpolated: bool,
    pub times: CircularBuffer<f32>,
    pub values: CircularBuffer<T>,
    // never used in parsed data
    // pub type_cache: Option<RewindCurveType>,
    // pub last_index: Option<i32>,
}

impl<T: Clone> RewindCurve<T> {
    pub(crate) unsafe fn read_from(
        reader: &mut BinaryReader<&[u8]>,
        curve_type: RewindCurveType,
    ) -> Result<Self, ReplayError> {
        let count = reader.read_i32()? as usize;
        let interpolated = reader.read_boolean()?;

        let mut times = CircularBuffer::with_capacity(count);
        for _ in 0..count {
            times.push_back(reader.read_f32()?);
        }

        let raw_values = reader.read_bytes(count * curve_type.size())?;
        let mut values = CircularBuffer::with_capacity(count);

        unsafe {
            for i in 0..count {
                let ptr = raw_values.as_ptr().add(i * curve_type.size());
                let value = std::ptr::read(ptr as *const T);
                values.push_back(value);
            }
        }

        Ok(Self {
            interpolated,
            times,
            values,
        })
    }
}
