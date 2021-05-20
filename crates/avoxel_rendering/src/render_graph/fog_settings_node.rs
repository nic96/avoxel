use crate::{fog_settings::FogSettings, render_graph::uniform};
use bevy::ecs::system::BoxedSystem;
use bevy::{
    core::AsBytes,
    ecs::system::IntoSystem,
    prelude::{Local, Res, ResMut, World},
    render::{
        render_graph::{CommandQueue, Node, ResourceSlots, SystemNode},
        renderer::{
            BufferId, BufferInfo, BufferMapMode, BufferUsage, RenderContext, RenderResourceBinding,
            RenderResourceBindings, RenderResourceContext,
        },
    },
};

#[derive(Debug)]
pub struct FogSettingsNode {
    command_queue: CommandQueue,
}

impl FogSettingsNode {
    pub fn new() -> Self {
        Self {
            command_queue: Default::default(),
        }
    }
}

impl Node for FogSettingsNode {
    fn update(
        &mut self,
        _world: &World,
        render_context: &mut dyn RenderContext,
        _input: &ResourceSlots,
        _output: &mut ResourceSlots,
    ) {
        self.command_queue.execute(render_context);
    }
}

impl SystemNode for FogSettingsNode {
    fn get_system(&self) -> BoxedSystem {
        let system = fog_settings_node_system.system().config(|config| {
            config.0 = Some(FogSettingsNodeState {
                command_queue: self.command_queue.clone(),
                fog_settings_buffer: None,
                staging_buffer: None,
            })
        });
        Box::new(system)
    }
}

#[derive(Debug, Default)]
pub struct FogSettingsNodeState {
    command_queue: CommandQueue,
    fog_settings_buffer: Option<BufferId>,
    staging_buffer: Option<BufferId>,
}

pub fn fog_settings_node_system(
    mut state: Local<FogSettingsNodeState>,
    render_resource_context: Res<Box<dyn RenderResourceContext>>,
    // PERF: this write on RenderResourceAssignments will prevent this system from running in parallel
    // with other systems that do the same
    mut render_resource_bindings: ResMut<RenderResourceBindings>,
    fog_settings: Res<FogSettings>,
) {
    if !fog_settings.is_changed() {
        return;
    };
    let fog_color_size = std::mem::size_of::<[f32; 4]>();
    let fog_near_size = std::mem::size_of::<f32>();
    let fog_far_size = std::mem::size_of::<f32>();
    let fog_settings_uniform_size = fog_color_size + fog_near_size + fog_far_size;
    let staging_buffer = if let Some(staging_buffer) = state.staging_buffer {
        render_resource_context.map_buffer(staging_buffer, BufferMapMode::Write);
        staging_buffer
    } else {
        let buffer = render_resource_context.create_buffer(BufferInfo {
            size: fog_settings_uniform_size,
            buffer_usage: BufferUsage::COPY_DST | BufferUsage::UNIFORM,
            ..Default::default()
        });
        render_resource_bindings.set(
            uniform::FOG_SETTINGS,
            RenderResourceBinding::Buffer {
                buffer,
                range: 0..fog_settings_uniform_size as u64,
                dynamic_index: None,
            },
        );
        state.fog_settings_buffer = Some(buffer);

        let staging_buffer = render_resource_context.create_buffer(BufferInfo {
            size: fog_settings_uniform_size,
            buffer_usage: BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
            mapped_at_creation: true,
        });

        state.staging_buffer = Some(staging_buffer);
        staging_buffer
    };

    let fog_color: [f32; 4] = fog_settings.fog_color.as_linear_rgba_f32();
    render_resource_context.write_mapped_buffer(
        staging_buffer,
        0..fog_settings_uniform_size as u64,
        &mut |data, _renderer| {
            // fog color
            data[0..fog_color_size].copy_from_slice(fog_color.as_bytes());
            // fog near
            data[fog_color_size..fog_color_size + fog_near_size]
                .copy_from_slice(fog_settings.fog_near.as_bytes());
            // fog far
            data[fog_color_size + fog_near_size..fog_settings_uniform_size]
                .copy_from_slice(fog_settings.fog_far.as_bytes());
        },
    );
    render_resource_context.unmap_buffer(staging_buffer);
    let fog_settings_buffer = state.fog_settings_buffer.unwrap();
    state.command_queue.copy_buffer_to_buffer(
        staging_buffer,
        0,
        fog_settings_buffer,
        0,
        fog_settings_uniform_size as u64,
    );
}
