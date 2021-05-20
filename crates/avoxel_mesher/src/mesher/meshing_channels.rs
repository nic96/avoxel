use avoxel_math::Pos;
use bevy::prelude::Mesh;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::time::Instant;

pub struct MeshingChannels {
    /// Sending Instant for timing purposes
    pub(crate) tx: Sender<(Pos, Mesh, Instant)>,
    pub(crate) rx: Receiver<(Pos, Mesh, Instant)>,
}

impl Default for MeshingChannels {
    fn default() -> Self {
        let (tx, rx) = unbounded();
        Self { tx, rx }
    }
}
