use bevy_app::{PluginGroup, PluginGroupBuilder};

pub struct AvoxelDefaultPlugins;

impl PluginGroup for AvoxelDefaultPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(avoxel_chunk_map::AvoxelChunkMapPlugin);
        group.add(avoxel_physics::AvoxelPhysicsPlugin);
        #[cfg(feature = "avoxel_mesher")]
        group.add(avoxel_mesher::AvoxelMesherPlugin);
    }
}
