use crate::{
    chunk_map::{ChunkMap, ChunkState},
    chunk_map_diagnostics::{CHUNK_COMPRESSION, COMPRESSION_TIMES, GEN_TIMES},
    chunk_viewer::{ChunkViewer, ChunkViewerMoveEvent},
};
use avoxel_blocks::BlockLibrary;
use avoxel_generator::generator::generate_chunk;
use bevy::{diagnostic::Diagnostics, prelude::*, tasks::AsyncComputeTaskPool};
use parking_lot::Mutex;
use std::{mem::size_of, sync::Arc, time::Instant};

pub fn update_block_library(block_library: Res<BlockLibrary>, mut chunk_map: ResMut<ChunkMap>) {
    if !block_library.is_changed() {
        return;
    }
    chunk_map.block_library = Arc::new(block_library.clone());
    chunk_map.dirty_chunks = chunk_map.visible_chunks.clone();
}

pub fn update_visible_chunks(
    mut chunk_map: ResMut<ChunkMap>,
    mut move_event_reader: EventReader<ChunkViewerMoveEvent>,
    viewers: Query<&ChunkViewer>,
) {
    if let Some(_event) = move_event_reader.iter().last() {
        chunk_map.visible_chunks.clear();
        for viewer in viewers.iter() {
            chunk_map.visible_chunks.extend(viewer.get_visible_chunks());
        }
        // get a list of chunks to remove
        let mut chunks_to_remove = vec![];
        for pos in chunk_map.chunk_keys() {
            if !chunk_map.visible_chunks.contains(pos) {
                chunks_to_remove.push(*pos);
            }
        }
        // remove chunks
        for pos in &chunks_to_remove {
            chunk_map.remove_chunk(pos);
        }
    }
}

pub fn gen_chunks_system(
    pool: Res<AsyncComputeTaskPool>,
    mut chunk_map: ResMut<ChunkMap>,
    mut diagnostics: ResMut<Diagnostics>,
) {
    let visible_chunks = chunk_map.visible_chunks.clone();
    for pos in &visible_chunks {
        match chunk_map.chunk_state(&pos.clone()) {
            ChunkState::Unloaded => {}
            _ => continue,
        }
        chunk_map.set_chunk_state_loading(&pos.clone());
        let sender = chunk_map.gen_channels.tx.clone();
        let pos = *pos;
        pool.spawn(async move {
            let start_instant = Instant::now();
            sender
                .send((generate_chunk(&pos), start_instant))
                .expect("Failed to send chunk");
        })
        .detach();
    }

    let receiver = chunk_map.gen_channels.rx.clone();
    for (chunk, start_instant) in receiver.try_iter() {
        let pos = chunk.pos;
        chunk_map.set_chunk_state_loaded(&pos);
        chunk_map
            .chunks
            .insert(chunk.pos, Arc::new(Mutex::new(chunk)));
        diagnostics.add_measurement(GEN_TIMES, start_instant.elapsed().as_secs_f64());
        if cfg!(feature = "mesher") {
            chunk_map.make_dirty(&pos);
        } else {
            // if not meshing chunk we can compress the chunk already
            chunk_map.compress_chunk(pool.clone(), pos);
        }
    }
}

pub fn store_decompressed_compressed_chunks(
    mut chunk_map: ResMut<ChunkMap>,
    mut diagnostics: ResMut<Diagnostics>,
) {
    for chunk in chunk_map.decompression_channels.rx.clone().try_iter() {
        let chunk_key = chunk.lock().pos;
        // if the compressed version fo the chunk was not already removed we remove it
        // and store the decompressed chunk.
        if chunk_map.compressed_chunks.remove(&chunk_key).is_some() {
            chunk_map.chunks.insert(chunk_key, chunk);
        }
    }

    // store compressed chunks
    for (compressed_chunk, start_instant) in chunk_map.compression_channels.rx.clone().try_iter() {
        let compressed_size =
            compressed_chunk.compressed_voxels.len() as f64 * size_of::<u8>() as f64;
        // TODO: make sure chunk hasn't changed since compression began. If chunk changed just discard the compressed chunk.
        chunk_map.remove_chunk(&compressed_chunk.pos);
        chunk_map
            .compressed_chunks
            .insert(compressed_chunk.pos, compressed_chunk);
        diagnostics.add_measurement(COMPRESSION_TIMES, start_instant.elapsed().as_secs_f64());
        diagnostics.add_measurement(CHUNK_COMPRESSION, 1_149_984_f64 / compressed_size);
    }
}
