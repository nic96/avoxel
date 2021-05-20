use crate::mesher::*;
use avoxel_blocks::BlockLibrary;
use avoxel_chunk::*;
use avoxel_chunk_map::{chunk_map_diagnostics::MESH_TIMES, ChunkMap, ChunkViewer};
use avoxel_math::*;
use avoxel_rendering::AvoxelChunkBundle;
use bevy::{diagnostic::Diagnostics, prelude::*, tasks::AsyncComputeTaskPool};
use std::time::Instant;

#[allow(clippy::too_many_arguments)]
pub fn mesh_dirty_chunks(
    mut commands: Commands,
    pool: Res<AsyncComputeTaskPool>,
    mut chunk_map: ResMut<ChunkMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut diagnostics: ResMut<Diagnostics>,
    block_library: Res<BlockLibrary>,
    mut mesher: ResMut<Mesher>,
    _viewers: Query<&ChunkViewer>,
) {
    // spawn meshing tasks for dirty chunks
    for pos in &chunk_map.dirty_chunks {
        let sender = mesher.meshing_channels.tx.clone();
        let block_library = chunk_map.block_library.clone();
        match chunk_map.chunks.get(pos) {
            None => match chunk_map.compressed_chunks.get(pos) {
                None => continue,
                Some(c) => {
                    let chunk = c.decompress(chunk_map.get_byteorder());
                    pool.spawn(async move {
                        let start_instant = Instant::now();
                        if let Some(mesh) = generate_mesh_culled(&chunk, block_library) {
                            match sender.send((chunk.pos, mesh, start_instant)) {
                                Ok(_) => {}
                                Err(e) => {
                                    warn!(
                                        "failed to send mesh with channel: {}",
                                        e.0 .0.to_string()
                                    );
                                }
                            }
                        }
                    })
                    .detach();
                }
            },
            Some(c) => {
                let chunk = c.clone();
                pool.spawn(async move {
                    let chunk = chunk.lock();
                    let start_instant = Instant::now();
                    if let Some(mesh) = generate_mesh_culled(&chunk, block_library) {
                        match sender.send((chunk.pos, mesh, start_instant)) {
                            Ok(_) => {}
                            Err(e) => {
                                warn!("failed to send mesh with channel: {}", e.0 .0.to_string());
                            }
                        }
                    }
                })
                .detach();
            }
        };
    }
    if !chunk_map.dirty_chunks.is_empty() {
        chunk_map.dirty_chunks.clear();
    }

    // spawn avoxel chunk bundles for completed chunk meshes
    let receiver = mesher.meshing_channels.rx.clone();
    for (pos, mesh, start_instant) in receiver.try_iter().take(12) {
        if chunk_map.contains_chunk(&pos) {
            let current_entity = commands
                .spawn()
                .insert_bundle(AvoxelChunkBundle {
                    mesh: meshes.add(mesh),
                    material: block_library.get_material_handle(0).clone(),
                    transform: Transform::from_translation((pos * CHUNK_SIZE).to_vec3()),
                    ..Default::default()
                })
                .id();

            match mesher.mesh_entities.get(&pos) {
                None => {}
                Some(entities) => {
                    for entity in entities {
                        commands.entity(*entity).despawn_recursive();
                    }
                }
            }
            mesher.mesh_entities.insert(pos, vec![current_entity]);
            diagnostics.add_measurement(MESH_TIMES, start_instant.elapsed().as_secs_f64());

            // compress chunks after meshing is complete
            chunk_map.compress_chunk(pool.clone(), pos);
        }
    }

    // de-spawn chunk meshes
    let mut positions = vec![];
    for pos in mesher.mesh_entities.keys() {
        if !chunk_map.visible_chunks.contains(pos) {
            positions.push(*pos);
        }
    }

    for pos in positions {
        if let Some(entities) = mesher.mesh_entities.remove(&pos) {
            for entity in entities {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
