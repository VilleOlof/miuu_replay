use csharp_binary_encoding::BinaryReader;
use miniz_oxide::inflate::decompress_to_vec;

use crate::{
    Elevator, Marble, Powerup, Replay, ReplayError, Rewindable, RewindableData, Vector3,
    objects::bumper::Bumper,
};

impl Replay {
    /// Decodes the raw `replay_buffer` within a [`Replay`] to a [`ReplayBuffer`]
    pub fn decode_replay_buffer(&self) -> Result<ReplayBuffer, ReplayError> {
        let mut reader = BinaryReader::new(self.data.replay_buffer.as_slice());
        let header = ReplayHeader {
            session: reader.read_i32()?,
            version: reader.read_i32()?,
        };

        let compressed_len = self.data.replay_buffer.len() - reader.num_bytes_read() as usize;
        let compressed_data = reader.read_bytes(compressed_len)?;
        let data = decompress_to_vec(&compressed_data)?;
        let mut reader = BinaryReader::new(data.as_slice());

        let rewindable_count = reader.read_i32()?;
        let mut rewindables = Vec::with_capacity(rewindable_count as usize);

        for _ in 0..rewindable_count {
            let game_object_name = reader.read_string()?;
            let type_name = reader.read_string()?;
            let ref_pos =
                Vector3::new((reader.read_f32()?, reader.read_f32()?, reader.read_f32()?));

            let data = RewindableData::read_from(&mut reader)?;

            let rewindable = Rewindable {
                game_object_name,
                type_name,
                ref_pos,
                data,
            };

            rewindables.push(rewindable);
        }

        Ok(ReplayBuffer {
            header,
            rewindable_count,
            rewindables,
        })
    }
}

/// A fully decoded `replay_buffer`
#[derive(Debug, Clone)]
pub struct ReplayBuffer {
    pub header: ReplayHeader,
    pub rewindable_count: i32,
    pub rewindables: Vec<Rewindable>,
}

/// The header information for a [`ReplayBuffer`]
#[derive(Debug, Clone)]
pub struct ReplayHeader {
    pub session: i32,
    pub version: i32,
}

impl ReplayBuffer {
    /// The `type_name` of the [`Rewindable`] that is the player marble object
    pub(crate) const MARBLE_CONTROLLER: &'static str = "MarbleController";

    /// The `type_name` of the [`Rewindable`] that is any powerup
    pub(crate) const POWERUP: &'static str = "Powerup";

    /// The `type_name` of the [`Rewindable`] that is bumpers
    pub(crate) const BUMPER_CONTROLLER: &'static str = "BumperController";

    /// The `type_name` of the [`Rewindable`] that is elevator movers
    pub(crate) const ELEVATOR_MOVER: &'static str = "ElevatorMover";

    /// Get's the [`Rewindable`] with the type name of [`MARBLE_CONTROLLER`](Self::MARBLE_CONTROLLER).  
    ///
    /// Which is the actual player object within the replay.  
    ///
    /// This operation **removes** the [`Rewindable`] from the buffer and returns it.  
    pub fn get_marble(&mut self) -> Result<Marble, ReplayError> {
        let idx = self
            .rewindables
            .iter()
            .position(|r| r.type_name == Self::MARBLE_CONTROLLER)
            .ok_or(ReplayError::NoMarbleController)?;

        Ok(Marble {
            inner: self.rewindables.remove(idx),
        })
    }

    /// Get's all [`Rewindable`]'s with a type name of [`POWERUP`](Self::POWERUP)
    ///
    /// This operation **removes** the [`Rewindable`] from the buffer and returns it.  
    pub fn get_powerups(&mut self) -> Result<Vec<Powerup>, ReplayError> {
        let mut powerups = Vec::new();

        let mut i = 0;
        loop {
            if i == self.rewindables.len() {
                break;
            }

            if &self.rewindables[i].type_name == Self::POWERUP {
                powerups.push(Powerup {
                    inner: self.rewindables.remove(i),
                });
            } else {
                i += 1;
            }
        }

        Ok(powerups)
    }

    /// Get's all [`Rewindable`]'s with a type name of [`BUMPER_CONTROLLER`](Self::BUMPER_CONTROLLER)
    ///
    /// This operation **removes** the [`Rewindable`] from the buffer and returns it.  
    pub fn get_bumpers(&mut self) -> Result<Vec<Bumper>, ReplayError> {
        let mut bumpers = Vec::new();

        let mut i = 0;
        loop {
            if i == self.rewindables.len() {
                break;
            }

            if &self.rewindables[i].type_name == Self::BUMPER_CONTROLLER {
                bumpers.push(Bumper {
                    inner: self.rewindables.remove(i),
                });
            } else {
                i += 1;
            }
        }

        Ok(bumpers)
    }

    /// Get's all [`Rewindable`]'s with a type name of [`ELEVATOR_MOVER`](Self::ELEVATOR_MOVER)
    ///
    /// This operation **removes** the [`Rewindable`] from the buffer and returns it.  
    pub fn get_elevators(&mut self) -> Result<Vec<Elevator>, ReplayError> {
        let mut elevators = Vec::new();

        let mut i = 0;
        loop {
            if i == self.rewindables.len() {
                break;
            }

            if &self.rewindables[i].type_name == Self::ELEVATOR_MOVER {
                elevators.push(Elevator {
                    inner: self.rewindables.remove(i),
                });
            } else {
                i += 1;
            }
        }

        Ok(elevators)
    }
}
