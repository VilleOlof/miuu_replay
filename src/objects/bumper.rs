use crate::{RewindCurveFitter, Rewindable, field};

/// A wrapper type for [`Rewindable`] that comes from [`ReplayBuffer::get_bumpers`](crate::ReplayBuffer::get_bumpers).  
#[derive(Debug, Clone)]
pub struct Bumper {
    pub inner: Rewindable,
}

impl Bumper {
    field!(
        strike_time_left,
        &RewindCurveFitter<f32>,
        "StrikeTimeLeft",
        Float
    );
}
