use crate::mesher::{mesh_tables, mesh_tables::*};
use avoxel_blocks::{Block, BlockLibrary};
use avoxel_chunk::*;
use avoxel_math::{BevyVec3, Extent3};
use bevy::{
    prelude::Mesh,
    render::{
        mesh::{Indices, VertexAttributeValues},
        pipeline::PrimitiveTopology,
    },
};
use std::sync::Arc;

const ATTRIBUTE_TEXTURE_DATUM: &str = "Texture_Datum";

pub fn generate_mesh_culled(chunk: &Chunk, block_library: Arc<BlockLibrary>) -> Option<Mesh> {
    if chunk.is_empty() {
        return None;
    }
    // mesh storages
    let mut vertices: Vec<[f32; 3]> = vec![];
    let mut normals: Vec<[f32; 3]> = vec![];
    let mut texture_data: Vec<u32> = vec![];
    let mut indices: Vec<u32> = vec![];

    // keep track of vertices needed for indices
    let mut vert_count = 0;

    let mut extend_mesh = |square: &Square| {
        vertices.extend(&square.verts);
        normals.extend(&square.norms);
        texture_data.extend(&square.texture_data);
        indices.extend(&mesh_tables::indices(vert_count));
        vert_count += 4;
    };

    let extent = chunk.extent();
    let sub_extent = Extent3 {
        min: extent.min + CHUNK_PADDING,
        max: extent.max - CHUNK_PADDING,
    };
    let start_index = chunk.block_index(sub_extent.min);
    let end_index = chunk.block_index(sub_extent.max) + 1;
    for (i, v) in chunk
        .voxels
        .iter()
        .enumerate()
        .skip(start_index)
        .take(end_index - start_index)
    {
        let block_pos = chunk.index_to_pos(i);
        if *v == Block::AIR {
            continue;
        }
        if !sub_extent.contains_point(block_pos) {
            continue;
        }

        let block = block_library.get_block(*v as usize);

        let local_block_pos = (block_pos - chunk.extent().min - CHUNK_PADDING).to_vec3();

        let neighbor_left = chunk.voxels[i - CHUNK_LAYER_SIZE_WITH_PADDING as usize];
        if is_face_visible(&neighbor_left) {
            let tex_id = block.texture_ids[Block::LEFT];
            extend_mesh(&square_left(
                local_block_pos,
                tex_id,
                block_library.get_block_texture(tex_id).rand_rot,
            ));
        }

        let neighbor_back = chunk.voxels[i - CHUNK_SIZE_WITH_PADDING as usize];
        if is_face_visible(&neighbor_back) {
            let tex_id = block.texture_ids[Block::BACK];
            extend_mesh(&square_back(
                local_block_pos,
                tex_id,
                block_library.get_block_texture(tex_id).rand_rot,
            ));
        }

        let neighbor_bottom = chunk.voxels[i - 1];
        if is_face_visible(&neighbor_bottom) {
            let tex_id = block.texture_ids[Block::BOTTOM];
            extend_mesh(&square_bottom(
                local_block_pos,
                tex_id,
                block_library.get_block_texture(tex_id).rand_rot,
            ));
        }

        let neighbor_top = chunk.voxels[i + 1];
        if is_face_visible(&neighbor_top) {
            let tex_id = block.texture_ids[Block::TOP];
            extend_mesh(&square_top(
                local_block_pos,
                tex_id,
                block_library.get_block_texture(tex_id).rand_rot,
            ));
        }

        let neighbor_front = chunk.voxels[i + CHUNK_SIZE_WITH_PADDING as usize];
        if is_face_visible(&neighbor_front) {
            let tex_id = block.texture_ids[Block::FRONT];
            extend_mesh(&square_front(
                local_block_pos,
                tex_id,
                block_library.get_block_texture(tex_id).rand_rot,
            ));
        }

        let neighbor_right = chunk.voxels[i + CHUNK_LAYER_SIZE_WITH_PADDING as usize];
        if is_face_visible(&neighbor_right) {
            let tex_id = block.texture_ids[Block::RIGHT];
            extend_mesh(&square_right(
                local_block_pos,
                tex_id,
                block_library.get_block_texture(tex_id).rand_rot,
            ));
        }
    }

    if vertices.is_empty() {
        return None;
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(vertices),
    );
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));
    mesh.set_attribute(
        ATTRIBUTE_TEXTURE_DATUM,
        VertexAttributeValues::from(texture_data),
    );
    mesh.set_indices(Some(Indices::U32(indices)));
    Some(mesh)
}

fn is_face_visible(neighbor: &u32) -> bool {
    *neighbor == Block::AIR
}
