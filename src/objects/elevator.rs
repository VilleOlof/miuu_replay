use crate::{RewindCurveFitter, Rewindable, field};

/// A wrapper type for [`Rewindable`] that comes from [`ReplayBuffer::get_elevators`](crate::ReplayBuffer::get_elevators).  
#[derive(Debug, Clone)]
pub struct Elevator {
    pub inner: Rewindable,
}

impl Elevator {
    field!(t, &RewindCurveFitter<i32>, "T", Int);
    field!(collapsing, &RewindCurveFitter<bool>, "Collapsing", Bool);
    field!(stop_time, &RewindCurveFitter<f32>, "StopTime", Float);
    field!(enable_bob, &RewindCurveFitter<bool>, "EnableBob", Bool);
    field!(global_time, &RewindCurveFitter<i32>, "GlobalTime", Int);
}
