use avoxel_math::Pos;
use bevy::prelude::Vec3;

pub struct VoxelRayCastResult {
    pub hit_pos: Vec3,
    pub hit_norm: Vec3,
    pub block_pos: Pos,
}

fn trace_ray(
    mut predicate: impl FnMut(Pos) -> bool,
    ox: f32,
    oy: f32,
    oz: f32,
    dx: f32,
    dy: f32,
    dz: f32,
    max_d: f32,
) -> Option<VoxelRayCastResult> {
    let mut hit_pos = Vec3::ZERO;
    let mut hit_norm = Vec3::ZERO;

    // implementation based on the following paper:
    // http://www.cse.chalmers.se/edu/year/2010/course/TDA361/grid.pdf
    let mut t = 0.0;
    let mut ix = ox.floor() as i32;
    let mut iy = oy.floor() as i32;
    let mut iz = oz.floor() as i32;

    let stepx = if dx > 0. { 1 } else { -1 };
    let stepy = if dy > 0. { 1 } else { -1 };
    let stepz = if dz > 0. { 1 } else { -1 };

    // dx,dy,dz are already normalized
    let tx_delta = (1. / dx).abs();
    let ty_delta = (1. / dy).abs();
    let tz_delta = (1. / dz).abs();

    let xdist = if stepx > 0 {
        ix as f32 + 1. - ox
    } else {
        ox - ix as f32
    };
    let ydist = if stepy > 0 {
        iy as f32 + 1. - oy
    } else {
        oy - iy as f32
    };
    let zdist = if stepz > 0 {
        iz as f32 + 1. - oz
    } else {
        oz - iz as f32
    };

    // location of nearest voxel boundary, in units of t
    let mut tx_max = if tx_delta < f32::INFINITY {
        tx_delta * xdist
    } else {
        f32::INFINITY
    };
    let mut ty_max = if ty_delta < f32::INFINITY {
        ty_delta * ydist
    } else {
        f32::INFINITY
    };
    let mut tz_max = if tz_delta < f32::INFINITY {
        tz_delta * zdist
    } else {
        f32::INFINITY
    };

    let mut stepped_index = -1;

    // main loop along raycast vector
    while t <= max_d {
        // exit check
        if predicate(Pos::new(ix, iy, iz)) {
            hit_pos.x = ox + t * dx;
            hit_pos.y = oy + t * dy;
            hit_pos.z = oz + t * dz;

            hit_norm.x = 0.;
            hit_norm.y = 0.;
            hit_norm.z = 0.;
            if stepped_index == 0 {
                hit_norm.x = -stepx as f32;
            }
            if stepped_index == 1 {
                hit_norm.y = -stepy as f32;
            }
            if stepped_index == 2 {
                hit_norm.z = -stepz as f32;
            }

            return Some(VoxelRayCastResult {
                hit_pos,
                hit_norm,
                block_pos: Pos::new(ix, iy, iz),
            });
        };

        // advance t to next nearest voxel boundary
        if tx_max < ty_max {
            if tx_max < tz_max {
                ix += stepx;
                t = tx_max;
                tx_max += tx_delta;
                stepped_index = 0;
            } else {
                iz += stepz;
                t = tz_max;
                tz_max += tz_delta;
                stepped_index = 2;
            }
        } else if ty_max < tz_max {
            iy += stepy;
            t = ty_max;
            ty_max += ty_delta;
            stepped_index = 1;
        } else {
            iz += stepz;
            t = tz_max;
            tz_max += tz_delta;
            stepped_index = 2;
        }
    }

    // no voxel hit found
    hit_pos.x = ox + t * dx;
    hit_pos.y = oy + t * dy;
    hit_pos.z = oz + t * dz;
    hit_norm.x = 0.;
    hit_norm.y = 0.;
    hit_norm.z = 0.;

    return None;
}

pub fn voxel_ray_cast(
    predicate: impl FnMut(Pos) -> bool,
    origin: Vec3,
    direction: Vec3,
    max_d: f32,
) -> Option<VoxelRayCastResult> {
    let ox = origin.x;
    let oy = origin.y;
    let oz = origin.z;
    let mut dx = direction.x;
    let mut dy = direction.y;
    let mut dz = direction.z;
    let ds = (dx * dx + dy * dy + dz * dz).sqrt();

    if ds == 0. {
        panic!("Can't raycast along a zero vector");
    }

    dx /= ds;
    dy /= ds;
    dz /= ds;

    return trace_ray(predicate, ox, oy, oz, dx, dy, dz, max_d);
}
