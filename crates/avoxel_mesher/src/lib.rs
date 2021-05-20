use avoxel_rendering::AvoxelRenderingPlugin;
use bevy::prelude::*;

mod mesher;
mod state;
mod systems;

pub struct AvoxelMesherPlugin;

impl Plugin for AvoxelMesherPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AvoxelRenderingPlugin)
            .insert_resource(mesher::Mesher::default())
            .insert_resource(state::State::default())
            .add_system(systems::mesh_dirty_chunks.system())
            .add_system(state::handle_state_system.system());
    }
}
