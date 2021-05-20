mod mesh_tables;
mod mesher_culling;
mod meshing_channels;

use avoxel_math::Pos;
use bevy::{prelude::*, utils::HashMap};
pub use mesher_culling::generate_mesh_culled;
pub use meshing_channels::MeshingChannels;

#[derive(Default)]
pub struct Mesher {
    /// Mesh entities
    pub mesh_entities: HashMap<Pos, Vec<Entity>>,
    pub meshing_channels: MeshingChannels,
}
