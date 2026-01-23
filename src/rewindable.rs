use csharp_binary_encoding::BinaryReader;

use crate::{ReplayError, RewindCurveType, Vector3, rewind_curve::IRewindCurve};

#[derive(Debug, Clone)]
pub struct Rewindable {
    pub game_object_name: String,
    pub type_name: String,
    pub ref_pos: Vector3,
    pub data: Vec<RewindableData>,
}

#[derive(Debug, Clone)]
pub struct RewindableData {
    pub index: i32,
    pub text: String,
    pub rewind_type: RewindCurveType,
    pub curve: IRewindCurve,
}

impl RewindableData {
    pub(crate) fn read_from(
        reader: &mut BinaryReader<&[u8]>,
    ) -> Result<Vec<RewindableData>, ReplayError> {
        let num = reader.read_i32()?;

        let mut curves = Vec::with_capacity(num as usize);
        for _ in 0..num {
            let index = reader.read_i32()?;
            let text = reader.read_string()?;
            let _type = unsafe { RewindCurveType::from_i32(reader.read_i32()?) };

            let curve = _type.read_from(reader)?;
            curves.push(RewindableData {
                index,
                text,
                rewind_type: _type,
                curve,
            });
        }

        Ok(curves)
    }
}
