use crate::cube_cursor::cube_cursor_mesh;
use crate::player::FirstPersonCam;
use avoxel::{
    blocks::Block,
    math::{BevyVec3, Pos},
    prelude::ChunkMap,
};
use bevy::prelude::*;

pub struct PlayerInteractionPlugin;

impl Plugin for PlayerInteractionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_cursor.system())
            .add_system(cursor_position_system.system())
            .add_system(place_destroy_block_system.system());
    }
}

#[derive(Debug)]
struct TargetBlockCursor {
    pub pos: Pos,
    pub normal: Vec3,
}

/// Creates a box outline for the targeted block
fn setup_cursor(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let transform = Transform::from_translation(Vec3::new(0.0, 64., 0.0));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(cube_cursor_mesh()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0., 0., 0., 0.7),
                roughness: 1.0,
                reflectance: 0.0,
                unlit: true,
                ..Default::default()
            }),
            visible: Visible {
                is_visible: false,
                is_transparent: true,
            },
            transform,
            ..Default::default()
        })
        .insert(TargetBlockCursor {
            pos: Pos::default(),
            normal: Vec3::default(),
        });
}

fn cursor_position_system(
    chunk_map: Res<ChunkMap>,
    mut cursor: Query<(&mut Transform, &mut Visible, &mut TargetBlockCursor)>,
    camera: Query<&GlobalTransform, With<FirstPersonCam>>,
) {
    for (mut transform, mut visible, mut cursor) in cursor.iter_mut().take(1) {
        for cam_transform in camera.iter().take(1) {
            if let Some(voxel_ray_cast_result) =
                chunk_map.ray_cast(cam_transform.translation, -cam_transform.local_z(), 10.)
            {
                cursor.pos = voxel_ray_cast_result.block_pos;
                cursor.normal = voxel_ray_cast_result.hit_norm;
                visible.is_visible = true;
                transform.translation = voxel_ray_cast_result.block_pos.to_vec3();
            } else {
                visible.is_visible = false;
            }
        }
    }
}

fn place_destroy_block_system(
    mut chunk_map: ResMut<ChunkMap>,
    mouse_input: Res<Input<MouseButton>>,
    cursor: Query<(&TargetBlockCursor, &Visible)>,
    windows: Res<Windows>,
) {
    if let Some(window) = windows.get_primary() {
        if window.cursor_visible() {
            return;
        }
    }

    for (cursor, visible) in cursor.iter() {
        if visible.is_visible {
            if mouse_input.just_pressed(MouseButton::Left) {
                chunk_map.set_voxel(Block::AIR, &cursor.pos);
                return;
            }
            if mouse_input.just_pressed(MouseButton::Right) {
                chunk_map.set_voxel(2, &(cursor.pos + Pos::from_vec3(&cursor.normal)));
            }
        }
    }
}
