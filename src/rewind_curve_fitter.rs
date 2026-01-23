use std::fmt::Debug;

use csharp_binary_encoding::BinaryReader;

use crate::{ReplayError, rewind_curve::RewindCurve, rewind_curve_type::RewindCurveType};

#[derive(Debug, Clone)]
pub struct RewindCurveFitter<T: Clone> {
    pub interpolated: bool,
    pub recent_curve: RewindCurve<T>,
    // never used in parsed data
    // pub total_curve: RewindCurve<T>,
    // pub time: Option<f32>,
    // pub value: Option<T>,
}

impl<T: Clone> RewindCurveFitter<T> {
    pub(crate) fn read_from(
        reader: &mut BinaryReader<&[u8]>,
        curve_type: RewindCurveType,
    ) -> Result<Self, ReplayError> {
        let _type = unsafe { RewindCurveType::from_i32(reader.read_i32()?) };
        if _type != curve_type {
            return Err(ReplayError::MismatchedCurveTypes {
                lhs: _type,
                rhs: curve_type,
            });
        };

        let interpolated = reader.read_boolean()?;
        let recent_curve = unsafe { RewindCurve::<T>::read_from(reader, _type)? };

        Ok(Self {
            interpolated,
            recent_curve,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RewindCurveFitterArray<T: Clone> {
    pub interpolated: bool,
    pub count: i32,
    pub _type: RewindCurveType,
    // never used in parsed data
    // pub inner_type: RewindCurveType,
    pub curves: Vec<RewindCurveFitter<T>>,
}

impl<T: Clone> RewindCurveFitterArray<T> {
    pub(crate) fn read_from(
        reader: &mut BinaryReader<&[u8]>,
        curve_type: RewindCurveType,
    ) -> Result<Self, ReplayError> {
        let _type = unsafe { RewindCurveType::from_i32(reader.read_i32()?) };
        if _type != curve_type {
            return Err(ReplayError::MismatchedCurveTypes {
                lhs: _type,
                rhs: curve_type,
            });
        };

        let interpolated = reader.read_boolean()?;
        let count = reader.read_i32()?;
        let mut curves: Vec<RewindCurveFitter<T>> = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let mut fitter =
                RewindCurveFitter::<T>::read_from(reader, _type.get_curve_type_for_array_type())?;
            fitter.interpolated = interpolated;
            curves.push(fitter);
        }

        Ok(Self {
            interpolated,
            count,
            _type: _type,
            curves,
        })
    }
}
