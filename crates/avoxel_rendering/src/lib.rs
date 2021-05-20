use bevy::prelude::*;
use bevy::render::shader;

pub use entity::*;
use material::BlockMaterial;
pub use material::*;

use crate::fog_settings::FogSettings;
use crate::render_graph::add_avoxel_graph;

pub mod render_graph;

mod entity;
mod fog_settings;
mod material;

pub mod prelude {
    pub use crate::{entity::*, fog_settings::FogSettings, material::BlockMaterial};
}

#[derive(Default)]
pub struct AvoxelRenderingPlugin;

impl Plugin for AvoxelRenderingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<BlockMaterial>()
            .add_system_to_stage(
                CoreStage::PostUpdate,
                shader::asset_shader_defs_system::<BlockMaterial>.system(),
            )
            .init_resource::<FogSettings>();

        add_avoxel_graph(app.world_mut());

        // add default BlockMaterial
        let mut materials = app
            .world_mut()
            .get_resource_mut::<Assets<BlockMaterial>>()
            .unwrap();
        materials.set_untracked(
            Handle::<BlockMaterial>::default(),
            BlockMaterial {
                base_color: Color::PINK,
                unlit: true,
                ..Default::default()
            },
        );
    }
}
