mod channels;
mod chunk_map;
pub mod chunk_map_diagnostics;
mod chunk_viewer;
mod systems;
mod tools;

pub use crate::{chunk_map::ChunkMap, chunk_viewer::ChunkViewer};
use crate::{
    chunk_map_diagnostics::setup_diagnostics,
    chunk_viewer::{chunk_viewer_moved, ChunkViewerMoveEvent},
    systems::*,
};
use avoxel_blocks::BlockLibrary;
use bevy::prelude::*;

pub struct AvoxelChunkMapPlugin;

impl Plugin for AvoxelChunkMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        if app.world().get_resource::<BlockLibrary>().is_none() {
            app.init_resource::<BlockLibrary>();
        }
        if app.world().get_resource::<ChunkMap>().is_none() {
            app.init_resource::<ChunkMap>();
        }

        app.add_event::<ChunkViewerMoveEvent>()
            .add_startup_system(setup_diagnostics.system())
            .add_system(chunk_viewer_moved.system())
            .add_system(update_block_library.system())
            .add_system(update_visible_chunks.system())
            .add_system(gen_chunks_system.system())
            .add_system(store_decompressed_compressed_chunks.system());
    }
}
