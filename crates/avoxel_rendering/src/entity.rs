use crate::{material::BlockMaterial, render_graph::AVOXEL_PIPELINE_HANDLE};
use bevy::prelude::{
    Bundle, Draw, GlobalTransform, Handle, Mesh, RenderPipelines, Transform, Visible,
};
use bevy::render::pipeline::RenderPipeline;
use bevy::render::render_graph::base::MainPass;

/// A component bundle for "mesh of blocks" entities
/// This is basically the stuff needed to display a chunk
/// on screen. It contains the mesh, material, etc.
#[derive(Bundle)]
pub struct AvoxelChunkBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<BlockMaterial>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for AvoxelChunkBundle {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                AVOXEL_PIPELINE_HANDLE.typed(),
            )]),
            mesh: Default::default(),
            visible: Default::default(),
            material: Default::default(),
            main_pass: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}
