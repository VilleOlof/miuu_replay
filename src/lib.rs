use crate::rewind_curve_fitter::read_from_curve;
use csharp_binary_encoding::BinaryReader;

mod circular_buffer;
mod error;
mod quaternion;
mod replay;
mod rewind_curve;
mod rewind_curve_fitter;
mod rewind_curve_type;
mod rewindable;
mod vector2;
mod vector3;

pub use error::ReplayError;
pub use quaternion::Quaternion;
pub use replay::*;
pub use rewind_curve::RewindCurve;
pub use rewind_curve_fitter::{RewindCurveFitter, RewindCurveFitterArray};
pub use rewind_curve_type::RewindCurveType;
pub use rewindable::{Rewindable, RewindableData};
pub use vector2::Vector2;
pub use vector3::Vector3;

#[cfg(test)]
mod tests {
    use super::*;

    const REPLAY_FILE: &[u8] = include_bytes!("../test.replay");

    #[test]
    fn basic() {
        parse_replay(REPLAY_FILE).unwrap();
    }
}

fn read_rewindable_anim(reader: &mut BinaryReader<&[u8]>) -> Vec<RewindableData> {
    let num = reader.read_i32().unwrap();

    let mut curves = Vec::with_capacity(num as usize);
    for _ in 0..num {
        let index = reader.read_i32().unwrap();
        let text = reader.read_string().unwrap();
        let _type = RewindCurveType::from_i32(reader.read_i32().unwrap());

        let curve = read_from_curve(reader, _type);
        curves.push(RewindableData {
            index,
            text,
            rewind_type: _type,
            curve,
        });
    }

    curves
}

pub fn parse_replay(data: &[u8]) -> Result<Replay, ReplayError> {
    let replay: Replay = rmp_serde::from_slice(data).unwrap();

    println!("{replay:#?}");

    let mut reader = BinaryReader::new(replay.data.replay_buffer.as_slice());
    let rewind_session_header = reader.read_i32().unwrap();
    let rewind_session_version = reader.read_i32().unwrap();
    let header = ReplayHeader {
        session: rewind_session_header,
        version: rewind_session_version,
    };
    println!("{header:?}");

    let compressed_data = reader
        .read_bytes(replay.data.replay_buffer.len() - reader.num_bytes_read() as usize)
        .unwrap();
    let data = miniz_oxide::inflate::decompress_to_vec(&compressed_data).unwrap();
    let mut inner_reader = BinaryReader::new(data.as_slice());

    let rewindables = inner_reader.read_i32().unwrap();
    println!("Rewindables {{ {:?} }}", rewindables);

    let mut rewindables_data = Vec::with_capacity(rewindables as usize);

    for _ in 0..rewindables {
        let go_name = inner_reader.read_string().unwrap();
        let type_name = inner_reader.read_string().unwrap();
        let ref_pos = (
            inner_reader.read_f32().unwrap(),
            inner_reader.read_f32().unwrap(),
            inner_reader.read_f32().unwrap(),
        );
        let rewindable = Rewindable {
            go_name,
            type_name,
            ref_pos: Vector3::new(ref_pos),
        };

        println!("{rewindable:?}");
        rewindables_data.push(rewindable);

        let curves = read_rewindable_anim(&mut inner_reader);
        for curve in curves {
            println!("  > {curve:?}");
        }
    }

    Ok(replay)
}
