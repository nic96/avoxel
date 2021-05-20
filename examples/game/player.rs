use avoxel::math::Pos;
use avoxel::physics::{AvoxelBoxBuilder, AvoxelBoxHandleComponent, BoxMap};
use avoxel::prelude::ChunkViewer;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;
use nalgebra::{clamp, wrap};

const GRAVITY: f32 = 9.8;
const RENDER_DISTANCE: i32 = 4;

pub struct FirstPersonCam;

pub struct PlayerSettings {
    pub fly_normal_speed: f32,
    pub fly_sprint_speed: f32,
    pub speed: f32,
    pub sprint_speed: f32,
    pub mouse_sensitivity: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            fly_normal_speed: 20.0,
            fly_sprint_speed: 400.0,
            speed: 7.0,
            sprint_speed: 12.0,
            mouse_sensitivity: 0.1,
        }
    }
}

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut aabb = avoxel::math::Aabb::default();
    let translation = Vec3::new(0., 24., 0.);
    aabb.max.x = 0.9;
    aabb.max.z = 0.9;
    aabb.max.y = 1.9;
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(translation),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 1.0, 1.0),
                metallic: 0.0,
                roughness: 1.0,
                ..Default::default()
            }),
            mesh: asset_server.load("models/player/player.glb#Mesh0/Primitive0"),
            ..Default::default()
        })
        .insert(AvoxelBoxBuilder::new(translation, aabb))
        .with_children(|parent| {
            let mut camera_transform = Transform::from_translation(Vec3::new(0.45, 1.7, 0.45));
            camera_transform.rotation = Quat::from_rotation_ypr(0.0, 0.0, 0.);

            parent
                .spawn_bundle(PerspectiveCameraBundle {
                    transform: camera_transform,
                    perspective_projection: PerspectiveProjection {
                        near: 0.1,
                        fov: 70_f32.to_radians(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(FirstPersonCam);
        })
        .insert(ChunkViewer::new(RENDER_DISTANCE, Pos::default()));
}

pub struct PlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
    pub max_pitch: f32,
}

impl Default for PlayerRotation {
    fn default() -> Self {
        Self {
            yaw: 0_f32.to_radians(),
            pitch: 0_f32.to_radians(),
            max_pitch: 90_f32.to_radians(),
        }
    }
}

pub fn player_rotation_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    windows: Res<Windows>,
    player_settings: Res<PlayerSettings>,
    mut player_rot: Local<PlayerRotation>,
    mut camera: Query<&mut Transform, With<FirstPersonCam>>,
) {
    let window = windows.get_primary().unwrap();
    if window.cursor_visible() {
        return;
    }

    let mut rotation_move = Vec2::default();
    for ev in mouse_motion_events.iter() {
        rotation_move -= ev.delta * player_settings.mouse_sensitivity;
    }

    if rotation_move.length_squared() > 0.0 {
        // yaw
        player_rot.yaw = wrap(
            player_rot.yaw + rotation_move.x.to_radians(),
            -std::f32::consts::PI,
            std::f32::consts::PI,
        );
        // pitch
        player_rot.pitch = clamp(
            player_rot.pitch + rotation_move.y.to_radians(),
            -player_rot.max_pitch,
            player_rot.max_pitch,
        );
        for mut trans in camera.iter_mut() {
            trans.rotation = Quat::from_rotation_ypr(player_rot.yaw, player_rot.pitch, 0.);
        }
    }
}

pub fn player_movement_system(
    time: Res<Time>,
    mut box_map: ResMut<BoxMap>,
    input: Res<Input<KeyCode>>,
    player_settings: Res<PlayerSettings>,
    fp_cam: Query<&Transform, With<FirstPersonCam>>,
    query: Query<&AvoxelBoxHandleComponent>,
    mut chunk_viewer: Query<&mut ChunkViewer>,
) {
    let jump = input.pressed(KeyCode::Space);
    let speed;
    if input.pressed(KeyCode::LControl) {
        speed = player_settings.sprint_speed;
    } else {
        speed = player_settings.speed;
    }
    let mut input_dir = get_input_dir(input);
    for hc in query.iter() {
        if let Some(b) = box_map.get_mut(hc.handle()) {
            let mut velocity = b.velocity();
            if b.is_on_floor() && jump {
                velocity.y += 10.0;
            }
            velocity.y -= GRAVITY * time.delta_seconds() * 4.;
            if input_dir.length() > 0. {
                // Multiplying a quaternion and a 3D vector rotates the 3D vector.
                // this is done to convert the input direction which is a global direction
                // to the player's rotation.
                for tf in fp_cam.iter().take(1) {
                    // remove pitch from camera rotation
                    let player_rot = Quat::from_xyzw(0., tf.rotation.y, 0., tf.rotation.w);
                    input_dir = (player_rot * input_dir).normalize();
                }
                velocity.x = input_dir.x * speed;
                velocity.z = input_dir.z * speed;
            } else {
                velocity.x = 0.;
                velocity.z = 0.;
            }
            b.set_velocity(velocity);
            b.clamp_velocity(100.0);
            for mut c in chunk_viewer.iter_mut() {
                c.set_translation(b.translated_aabb().min);
            }
        }
    }
}

/// Get the direction player should moved based on keyboard input
fn get_input_dir(input: Res<Input<KeyCode>>) -> Vec3 {
    let mut input_dir = Vec3::default();
    let backward = Vec3::Z;
    if input.pressed(KeyCode::W) {
        input_dir -= backward;
    }
    if input.pressed(KeyCode::S) {
        input_dir += backward;
    }
    let right = Vec3::X;
    if input.pressed(KeyCode::A) {
        input_dir -= right;
    }
    if input.pressed(KeyCode::D) {
        input_dir += right;
    }
    input_dir
}

pub fn capture_mouse_system(
    mouse_input: Res<Input<MouseButton>>,
    input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
) {
    if let Some(window) = windows.get_primary_mut() {
        if window.cursor_visible() {
            if mouse_input.pressed(MouseButton::Left) {
                window.set_cursor_visibility(false);
                window.set_cursor_lock_mode(true);
            }
        } else {
            window.set_cursor_position(Vec2::new(window.width() / 2., window.height() / 2.));
            if input.pressed(KeyCode::Escape) {
                window.set_cursor_visibility(true);
                window.set_cursor_lock_mode(false);
            }
        }
    }
}
