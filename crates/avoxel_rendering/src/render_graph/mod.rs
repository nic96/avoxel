mod fog_settings_node;
mod pipeline;

pub use pipeline::*;

/// the names of rendering graph nodes
pub mod node {
    pub const BLOCK_MATERIAL: &str = "block_material";
    pub const FOG_SETTINGS: &str = "fog_settings";
}

pub mod uniform {
    pub const FOG_SETTINGS: &str = "FogSettings";
}

use crate::prelude::BlockMaterial;
use crate::render_graph::fog_settings_node::FogSettingsNode;
use bevy::prelude::{Assets, Shader, World};
use bevy::render::pipeline::PipelineDescriptor;
use bevy::render::render_graph::{base, AssetRenderResourcesNode, RenderGraph};

pub(crate) fn add_avoxel_graph(world: &mut World) {
    let mut graph = world.get_resource_mut::<RenderGraph>().unwrap();
    graph.add_system_node(
        node::BLOCK_MATERIAL,
        AssetRenderResourcesNode::<BlockMaterial>::new(true),
    );
    graph.add_system_node(node::FOG_SETTINGS, FogSettingsNode::new());
    graph
        .add_node_edge(node::BLOCK_MATERIAL, base::node::MAIN_PASS)
        .unwrap();
    let pipeline = build_pbr_pipeline(&mut world.get_resource_mut::<Assets<Shader>>().unwrap());
    let mut pipelines = world
        .get_resource_mut::<Assets<PipelineDescriptor>>()
        .unwrap();
    pipelines.set_untracked(AVOXEL_PIPELINE_HANDLE, pipeline);
}
