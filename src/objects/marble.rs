use std::ops::{Deref, DerefMut};

use crate::{Quaternion, RewindCurveFitter, RewindCurveFitterArray, Rewindable, Vector3, field};

/// A wrapper type for [`Rewindable`] that comes from [`ReplayBuffer::get_marble`](crate::ReplayBuffer::get_marble).  
///
/// This type ensures that it is in fact a `MarbleController` & exposes some *QoL* functions related to the marble
#[derive(Debug, Clone)]
pub struct Marble {
    pub inner: Rewindable,
}

impl Marble {
    field!(
        starting_remaining_ticks,
        &RewindCurveFitter<i32>,
        "StartingRemainingTicks",
        Int
    );
    field!(mode, &RewindCurveFitter<i32>, "Mode", Int);
    field!(
        invokable_effect_id,
        &RewindCurveFitter<u16>,
        "InvokableEffectId",
        UShort
    );
    field!(
        invokable_source_id,
        &RewindCurveFitter<u16>,
        "InvokableSourceId",
        UShort
    );
    field!(
        effect_state,
        &RewindCurveFitterArray<u32>,
        "EffectState",
        UInt32Array
    );
    field!(
        effect_ticks,
        &RewindCurveFitterArray<i32>,
        "EffectTicks",
        Int32Array
    );
    field!(qw, &RewindCurveFitter<Quaternion>, "qW", Quaternion);
    field!(
        gravity_quat,
        &RewindCurveFitter<Quaternion>,
        "GravityQuat",
        Quaternion
    );
    field!(position, &RewindCurveFitter<Vector3>, "Position", Vector3);
    field!(velocity, &RewindCurveFitter<Vector3>, "Velocity", Vector3);
    field!(omega, &RewindCurveFitter<Vector3>, "Omega", Vector3);
    field!(bonus_time, &RewindCurveFitter<f32>, "BonusTime", Float);
    field!(
        time_since_contact,
        &RewindCurveFitter<f32>,
        "TimeSinceContact",
        Float
    );
    field!(
        best_contact_normal,
        &RewindCurveFitter<Vector3>,
        "BestContactNormal",
        Vector3
    );
    field!(
        best_contact_surface_velocity,
        &RewindCurveFitter<Vector3>,
        "BestContactSurfaceVelocity",
        Vector3
    );
    field!(elapsed_time, &RewindCurveFitter<f32>, "ElapsedTime", Float);
    field!(
        collected_gems,
        &RewindCurveFitter<u16>,
        "CollectedGems",
        UShort
    );
    field!(
        mega_marble_size_scale,
        &RewindCurveFitter<f32>,
        "MegaMarbleSizeScale",
        Float
    );
    field!(
        done_first_bounce,
        &RewindCurveFitter<bool>,
        "DoneFirstBounce",
        Bool
    );
    field!(
        blast_cooldown,
        &RewindCurveFitter<f32>,
        "BlastCooldown",
        Float
    );
    field!(
        respawn_counter,
        &RewindCurveFitter<i32>,
        "RespawnCounter",
        Int
    );
}

impl Deref for Marble {
    type Target = Rewindable;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Marble {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
