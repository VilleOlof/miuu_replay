#![doc = include_str!("../readme.md")]

mod circular_buffer;
mod error;
mod objects;
mod quaternion;
mod replay;
mod replay_buffer;
mod rewind_curve;
mod rewind_curve_fitter;
mod rewind_curve_type;
mod rewindable;
mod vector2;
mod vector3;

pub use circular_buffer::CircularBuffer;
pub use error::ReplayError;
pub use objects::{bumper::Bumper, elevator::Elevator, marble::Marble, powerup::Powerup};
pub use quaternion::Quaternion;
pub use replay::*;
pub use replay_buffer::{ReplayBuffer, ReplayHeader};
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
    fn basic() -> Result<(), ReplayError> {
        let replay = Replay::parse(REPLAY_FILE)?;
        let mut buffer = replay.decode_replay_buffer()?;

        let _ = buffer.get_marble()?;

        Ok(())
    }
}
