use thiserror::Error;

use crate::{ReplayBuffer, RewindCurveType};

#[derive(Debug, Error)]
pub enum ReplayError {
    #[error("Mismatched curve types: {:?} != {:?}", .lhs, .rhs)]
    MismatchedCurveTypes {
        lhs: RewindCurveType,
        rhs: RewindCurveType,
    },
    #[error("Replay is missing a '{}' rewindable", ReplayBuffer::MARBLE_CONTROLLER)]
    NoMarbleController,
    #[error("{0} field is missing from the rewindable")]
    MissingField(&'static str),

    #[error("{0}")]
    DataDecode(#[from] csharp_binary_encoding::DataDecodeError),
    #[error("{0}")]
    MessagePack(#[from] rmp_serde::decode::Error),
    #[error("{0}")]
    Decompress(miniz_oxide::inflate::DecompressError),
}

impl From<miniz_oxide::inflate::DecompressError> for ReplayError {
    fn from(value: miniz_oxide::inflate::DecompressError) -> Self {
        ReplayError::Decompress(value)
    }
}
