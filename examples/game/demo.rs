use avoxel::prelude::*;
use bevy::prelude::*;

use crate::block_library::BlockLibraryPlugin;
use crate::hud::HudPlugin;
use crate::interaction::PlayerInteractionPlugin;
use crate::player::*;
use bevy::pbr::AmbientLight;

mod block_library;
mod cube_cursor;
mod generator;
mod hud;
mod interaction;
mod player;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Avoxel Demo".into(),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AmbientLight {
            color: Color::rgb_linear(0.3, 0.3, 0.5),
            brightness: 1.0,
        })
        .insert_resource(ClearColor(Color::rgb_u8(92, 119, 127)))
        // ChunkMap is the core of avoxel and to change terrain generation modify or change the generate chunk method
        .insert_resource(ChunkMap::new(false, &generator::generate_chunk))
        .add_plugins(DefaultPlugins)
        .add_plugin(BlockLibraryPlugin)
        .add_plugins(AvoxelDefaultPlugins)
        .add_plugin(HudPlugin)
        .add_plugin(PlayerInteractionPlugin)
        .add_startup_system(setup.system())
        .insert_resource(PlayerSettings::default())
        .add_startup_system(setup_player.system())
        .add_system(capture_mouse_system.system())
        .add_system(player_rotation_system.system())
        .add_system(player_movement_system.system())
        .add_system(toggle_fly.system())
        .run();
}

fn setup(mut commands: Commands) {
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(400.0, 800.0, 400.0)),
        point_light: PointLight {
            color: Color::rgb(0.8, 0.8, 0.8),
            intensity: 2000000.0,
            range: 10000000.0,
            ..Default::default()
        },
        ..Default::default()
    });
}
