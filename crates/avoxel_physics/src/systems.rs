use crate::{
    avoxel_box::{AvoxelBoxBuilder, AvoxelBoxChanges},
    box_map::BoxMap,
    components::AvoxelBoxHandleComponent,
    AvoxelPhysicsState,
};
use avoxel_blocks::Block;
use avoxel_chunk_map::ChunkMap;
use avoxel_math::{Aabb, BevyVec3, Pos};
use bevy::prelude::{Commands, Entity, Query, Res, ResMut, Time, Transform, Vec3};

pub fn create_avoxel_boxes_system(
    mut commands: Commands,
    mut box_map: ResMut<BoxMap>,
    query: Query<(Entity, &AvoxelBoxBuilder)>,
) {
    for (entity, bb) in query.iter() {
        let handle = box_map.insert(bb.build());
        commands
            .entity(entity)
            .insert(AvoxelBoxHandleComponent::from(handle));
        commands.entity(entity).remove::<AvoxelBoxBuilder>();
    }
}

/// Moves and slides all boxes based on the current velocity they have
pub fn box_move_and_slide(
    time: Res<Time>,
    mut box_map: ResMut<BoxMap>,
    chunk_map: Res<ChunkMap>,
    physics_state: Res<AvoxelPhysicsState>,
) {
    if physics_state.paused() {
        return;
    }
    for (_, b) in box_map.boxes.iter_mut() {
        let mut changed = true;
        let prev_aabb = b.translated_aabb();
        let prev_translation = b.translation;
        let linear_velocity = b.velocity() * time.delta_seconds();
        b.changes.insert(AvoxelBoxChanges::POSITION);
        b.translation += linear_velocity;
        b.on_floor = false;
        b.on_ceiling = false;
        // broad phase box
        let bpb = prev_aabb.get_swept_broad_phase_box(&linear_velocity);

        for x in bpb.min.x.floor() as i32..bpb.max.x.floor() as i32 + 1 {
            for y in bpb.min.y.floor() as i32..bpb.max.y.floor() as i32 + 1 {
                for z in bpb.min.z.floor() as i32..bpb.max.z.floor() as i32 + 1 {
                    let block_pos = Pos::new(x, y, z);
                    let block_aabb = Aabb {
                        min: block_pos.to_vec3(),
                        max: (block_pos + 1).to_vec3(),
                    };
                    let md = b.translated_aabb().minkowski_difference(&block_aabb);
                    if md.point_colliding(&Vec3::ZERO) {
                        if let Some(v) = chunk_map.get_voxel(&block_pos) {
                            if v != Block::AIR {
                                let pen_vec = md.penetration_vector(&Vec3::ZERO);
                                if pen_vec.y > 0. {
                                    if linear_velocity.y < 0. {
                                        b.on_floor = true;
                                    }
                                } else if pen_vec.y < 0. && linear_velocity.y > 0. {
                                    b.on_ceiling = true;
                                }
                                b.translation += pen_vec;
                            }
                        } else {
                            changed = false;
                            b.translation = prev_translation;
                        }
                    }
                }
            }
        }

        if b.on_floor || b.on_ceiling {
            b.velocity.y = 0.;
        }

        if changed {
            b.changes.insert(AvoxelBoxChanges::POSITION);
        }
    }
}

pub fn sync_transforms_system(
    box_map: Res<BoxMap>,
    mut query: Query<(&mut Transform, &AvoxelBoxHandleComponent)>,
) {
    for (mut transform, hc) in query.iter_mut() {
        if let Some(avoxel_box) = box_map.boxes.get(hc.handle()) {
            transform.translation = avoxel_box.translation;
        }
    }
}
