use std::fmt::Debug;

use csharp_binary_encoding::BinaryReader;

use crate::{circular_buffer::CircularBuffer, rewind_curve_type::RewindCurveType};

pub trait IRewindCurve: Debug {}

#[derive(Debug)]
pub struct RewindCurve<T: Clone> {
    pub interpolated: bool,
    pub type_cache: Option<RewindCurveType>,
    pub times: CircularBuffer<f32>,
    pub values: CircularBuffer<T>,
    pub last_index: Option<i32>,
}

impl<T: Clone> RewindCurve<T> {
    pub(crate) fn read_from(reader: &mut BinaryReader<&[u8]>, curve_type: RewindCurveType) -> Self {
        let count = reader.read_i32().unwrap() as usize;
        let interpolated = reader.read_boolean().unwrap();

        let mut times = CircularBuffer::with_capacity(count);
        for _ in 0..count {
            times.push_back(reader.read_f32().unwrap());
        }

        let raw_values = reader.read_bytes(count * curve_type.size()).unwrap();
        let mut values = CircularBuffer::with_capacity(count);

        unsafe {
            for i in 0..count {
                let ptr = raw_values.as_ptr().add(i * curve_type.size());
                let value = std::ptr::read(ptr as *const T);
                values.push_back(value);
            }
        }

        Self {
            interpolated,
            type_cache: None,
            times,
            values,
            last_index: None,
        }
    }
}

impl<T: Clone> Default for RewindCurve<T> {
    fn default() -> Self {
        Self {
            interpolated: true,
            type_cache: None,
            times: CircularBuffer::new(),
            values: CircularBuffer::new(),
            last_index: None,
        }
    }
}
