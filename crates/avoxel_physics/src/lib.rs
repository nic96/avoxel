use avoxel_chunk_map::ChunkMap;
use bevy::{
    app::AppBuilder,
    prelude::{IntoSystem, Plugin},
};

mod avoxel_box;
mod box_map;
mod components;
mod state;
mod systems;

pub use avoxel_box::AvoxelBoxBuilder;
pub use box_map::BoxMap;
pub use components::AvoxelBoxHandleComponent;
pub use state::*;

pub struct AvoxelPhysicsPlugin;

impl Plugin for AvoxelPhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        if !app.world().contains_resource::<ChunkMap>() {
            bevy::log::error!("AvoxelChunkMapPlugin needs to be added first.");
        }

        app.insert_resource(AvoxelPhysicsState::default())
            .insert_resource(BoxMap::default())
            .add_system(systems::create_avoxel_boxes_system.system())
            .add_system(systems::box_move_and_slide.system())
            .add_system(systems::sync_transforms_system.system());
    }
}
