use crate::{RewindCurveFitter, Rewindable, field};

/// A wrapper type for [`Rewindable`] that comes from [`ReplayBuffer::get_powerups`](crate::ReplayBuffer::get_powerups).  
#[derive(Debug, Clone)]
pub struct Powerup {
    pub inner: Rewindable,
}

impl Powerup {
    field!(
        available_for_pickup,
        &RewindCurveFitter<bool>,
        "AvailableForPickup",
        Bool
    );
    field!(point_value, &RewindCurveFitter<u16>, "PointValue", UShort);
}
