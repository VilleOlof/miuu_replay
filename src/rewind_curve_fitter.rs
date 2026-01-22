use std::fmt::Debug;

use csharp_binary_encoding::BinaryReader;

use crate::{
    quaternion::Quaternion,
    rewind_curve::{IRewindCurve, RewindCurve},
    rewind_curve_type::RewindCurveType,
    vector2::Vector2,
    vector3::Vector3,
};

pub(crate) fn read_from_curve(
    reader: &mut BinaryReader<&[u8]>,
    curve_type: RewindCurveType,
) -> Box<dyn IRewindCurve> {
    match curve_type {
        RewindCurveType::Unknown => unimplemented!(),
        RewindCurveType::Float => Box::new(RewindCurveFitter::<f32>::read_from(reader, curve_type)),
        RewindCurveType::Int => Box::new(RewindCurveFitter::<i32>::read_from(reader, curve_type)),
        RewindCurveType::Bool => Box::new(RewindCurveFitter::<bool>::read_from(reader, curve_type)),
        RewindCurveType::Vector2 => {
            Box::new(RewindCurveFitter::<Vector2>::read_from(reader, curve_type))
        }
        RewindCurveType::Vector3 => {
            Box::new(RewindCurveFitter::<Vector3>::read_from(reader, curve_type))
        }
        RewindCurveType::Quaternion => Box::new(RewindCurveFitter::<Quaternion>::read_from(
            reader, curve_type,
        )),
        RewindCurveType::UShort => {
            Box::new(RewindCurveFitter::<u16>::read_from(reader, curve_type))
        }
        RewindCurveType::UInt32 => {
            Box::new(RewindCurveFitter::<u32>::read_from(reader, curve_type))
        }
        RewindCurveType::UInt32Array => {
            Box::new(RewindCurveFitterArray::<u32>::read_from(reader, curve_type))
        }
        RewindCurveType::Int32Array => {
            Box::new(RewindCurveFitterArray::<i32>::read_from(reader, curve_type))
        }
    }
}

#[derive(Debug)]
pub struct RewindCurveFitter<T: Clone> {
    pub interpolated: bool,
    pub recent_curve: RewindCurve<T>,
    pub total_curve: RewindCurve<T>,
    pub time: Option<f32>,
    pub value: Option<T>,
}
impl<T: Clone> Default for RewindCurveFitter<T> {
    fn default() -> Self {
        Self {
            interpolated: true,
            recent_curve: RewindCurve::default(),
            total_curve: RewindCurve::default(),
            time: None,
            value: None,
        }
    }
}
impl<T: Clone + Debug> IRewindCurve for RewindCurveFitter<T> {}
impl<T: Clone> RewindCurveFitter<T> {
    pub(crate) fn read_from(reader: &mut BinaryReader<&[u8]>, curve_type: RewindCurveType) -> Self {
        let _type = RewindCurveType::from_i32(reader.read_i32().unwrap());
        if _type != curve_type {
            panic!("mismatched curve types: {_type:?} != {curve_type:?}")
        };

        let interpolated = reader.read_boolean().unwrap();
        let recent_curve = RewindCurve::<T>::read_from(reader, _type);

        Self {
            interpolated,
            recent_curve,
            total_curve: RewindCurve::default(),
            time: None,
            value: None,
        }
    }
}

#[derive(Debug)]
pub struct RewindCurveFitterArray<T: Clone> {
    pub interpolated: bool,
    pub count: i32,
    pub _type: RewindCurveType,
    pub inner_type: RewindCurveType,
    pub curves: Vec<RewindCurveFitter<T>>,
}
impl<T: Clone> Default for RewindCurveFitterArray<T> {
    fn default() -> Self {
        Self {
            interpolated: true,
            count: 1,
            _type: RewindCurveType::Unknown,
            inner_type: RewindCurveType::Unknown,
            curves: Vec::new(),
        }
    }
}
impl<T: Clone + Debug> IRewindCurve for RewindCurveFitterArray<T> {}
impl<T: Clone> RewindCurveFitterArray<T> {
    pub(crate) fn read_from(reader: &mut BinaryReader<&[u8]>, curve_type: RewindCurveType) -> Self {
        let _type = RewindCurveType::from_i32(reader.read_i32().unwrap());
        if _type != curve_type {
            panic!("mismatched curve array types: {_type:?} != {curve_type:?}")
        };

        let interpolated = reader.read_boolean().unwrap();
        let count = reader.read_i32().unwrap();
        let mut curves: Vec<RewindCurveFitter<T>> = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let mut fitter =
                RewindCurveFitter::<T>::read_from(reader, _type.get_curve_type_for_array_type());
            fitter.interpolated = interpolated;
            curves.push(fitter);
        }

        Self {
            interpolated,
            count,
            _type: _type,
            inner_type: _type,
            curves,
        }
    }
}
