use bevy::prelude::Vec3;

pub struct Square {
    pub verts: [[f32; 3]; 4],
    pub norms: [[f32; 3]; 4],
    pub texture_data: [u32; 4],
}

#[rustfmt::skip]
pub fn square_front(pos: Vec3, tex_id: u32, rand_rot: bool) -> Square {
    let x = pos.x;
    let x1 = x + 1.;
    let y = pos.y;
    let y1 = y + 1.;
    let z = pos.z;
    let z1 = z + 1.;

    let mut uvs = [
        [0., 1.],
        [1., 1.],
        [1., 0.],
        [0., 0.]
    ];
    if rand_rot {
        uvs = rotate_uvs(pos, uvs);
    }
    let texture_data = pack_texture_data(uvs, tex_id);

    Square {
        verts: [
            [x, y, z1],
            [x1, y, z1],
            [x1, y1, z1],
            [x, y1, z1],
        ],
        norms: [
            [0., 0., 1.0],
            [0., 0., 1.0],
            [0., 0., 1.0],
            [0., 0., 1.0]
        ],
        texture_data,
    }
}

#[rustfmt::skip]
pub fn square_back(pos: Vec3, tex_id: u32, rand_rot: bool) -> Square {
    let x = pos.x;
    let x1 = x + 1.;
    let y = pos.y;
    let y1 = y + 1.;
    let z = pos.z;

    let mut uvs = [
        [1., 0.],
        [0., 0.],
        [0., 1.],
        [1., 1.]
    ];
    if rand_rot {
        uvs = rotate_uvs(pos, uvs);
    }
    let texture_data = pack_texture_data(uvs, tex_id);

    Square {
        verts: [
            [x, y1, z],
            [x1, y1, z],
            [x1, y, z],
            [x, y, z],
        ],
        norms: [
            [0., 0., -1.],
            [0., 0., -1.],
            [0., 0., -1.],
            [0., 0., -1.]
        ],
        texture_data,
    }
}

#[rustfmt::skip]
pub fn square_right(pos: Vec3, tex_id: u32, rand_rot: bool) -> Square {
    let x = pos.x;
    let x1 = x + 1.;
    let y = pos.y;
    let y1 = y + 1.;
    let z = pos.z;
    let z1 = z + 1.;

    let mut uvs = [
        [1., 1.],
        [1., 0.],
        [0., 0.],
        [0., 1.]
    ];
    if rand_rot {
        uvs = rotate_uvs(pos, uvs);
    }
    let texture_data = pack_texture_data(uvs, tex_id);

    Square {
        verts: [
            [x1, y, z],
            [x1, y1, z],
            [x1, y1, z1],
            [x1, y, z1],
        ],
        norms: [
            [1., 0., 0.],
            [1., 0., 0.],
            [1., 0., 0.],
            [1., 0., 0.]
        ],
        texture_data,
    }
}

#[rustfmt::skip]
pub fn square_left(pos: Vec3, tex_id: u32, rand_rot: bool) -> Square {
    let x = pos.x;
    let y = pos.y;
    let y1 = y + 1.;
    let z = pos.z;
    let z1 = z + 1.;

    let mut uvs = [
        [1., 1.],
        [1., 0.],
        [0., 0.],
        [0., 1.]
    ];
    if rand_rot {
        uvs = rotate_uvs(pos, uvs);
    }
    let texture_data = pack_texture_data(uvs, tex_id);

    Square {
        verts: [
            [x, y, z1],
            [x, y1, z1],
            [x, y1, z],
            [x, y, z],
        ],
        norms: [
            [-1.0, 0., 0.],
            [-1.0, 0., 0.],
            [-1.0, 0., 0.],
            [-1.0, 0., 0.],
        ],
        texture_data,
    }
}

#[rustfmt::skip]
pub fn square_top(pos: Vec3, tex_id: u32, rand_rot: bool) -> Square {
    let x = pos.x;
    let x1 = x + 1.;
    let y = pos.y;
    let y1 = y + 1.;
    let z = pos.z;
    let z1 = z + 1.;

    let mut uvs = [
        [1., 0.],
        [0., 0.],
        [0., 1.],
        [1., 1.],
    ];
    if rand_rot {
        uvs = rotate_uvs(pos, uvs);
    }
    let texture_data = pack_texture_data(uvs, tex_id);

    Square {
        verts: [
            [x1, y1, z],
            [x, y1, z],
            [x, y1, z1],
            [x1, y1, z1],
        ],
        norms: [
            [0., 1., 0.],
            [0., 1., 0.],
            [0., 1., 0.],
            [0., 1., 0.],
        ],
        texture_data,
    }
}

#[rustfmt::skip]
pub fn square_bottom(pos: Vec3, tex_id: u32, rand_rot: bool) -> Square {
    let x = pos.x;
    let x1 = x + 1.;
    let y = pos.y;
    let z = pos.z;
    let z1 = z + 1.;

    let mut uvs = [
        [0., 0.],
        [1., 0.],
        [1., 1.],
        [0., 1.]
    ];
    if rand_rot {
        uvs = rotate_uvs(pos, uvs);
    }
    let texture_data = pack_texture_data(uvs, tex_id);

    Square {
        verts: [
            [x1, y, z1],
            [x, y, z1],
            [x, y, z],
            [x1, y, z],
        ],
        norms: [
            [0., -1., 0.],
            [0., -1., 0.],
            [0., -1., 0.],
            [0., -1., 0.]
        ],
        texture_data,
    }
}

pub const fn indices(count: u32) -> [u32; 6] {
    [
        0 + count,
        1 + count,
        2 + count,
        2 + count,
        3 + count,
        0 + count,
    ]
}

/// takes a set of uv coordinates and rotates them randomly in 90 degree increments
fn rotate_uvs(pos: Vec3, uvs: [[f32; 2]; 4]) -> [[f32; 2]; 4] {
    // seemingly random number generator based on this: https://stackoverflow.com/a/37221804/4103154
    let mut r = (pos.x as i32)
        .overflowing_mul(374761393)
        .0
        .overflowing_add((pos.y as i32).overflowing_mul(668265263).0)
        .0
        .overflowing_add((pos.z as i32).overflowing_mul(987643213).0)
        .0; // all constants are prime

    r = (r ^ (r >> 13)).overflowing_mul(1274126177).0;
    r = r ^ (r >> 16);
    // end of random number generator

    let rotation = (r % 4 * 90) as f32;

    let mut cx = 0.0;
    let mut cy = 0.0;
    // sum
    for uv in &uvs {
        cx += uv[0];
        cy += uv[1];
    }
    // average
    cx = cx / 4.;
    cy = cy / 4.;

    let angle = rotation.to_radians();
    let cos = angle.cos();
    let sin = angle.sin();
    let mut uvs = uvs;
    for uv in uvs.iter_mut() {
        let qx = cx + cos * (uv[0] - cx) - sin * (uv[1] - cy);
        uv[1] = (cy + sin * (uv[0] - cx) + cos * (uv[1] - cy)).round();
        uv[0] = qx.round();
    }
    uvs
}

/// Packs uvs and array texture index into 32 bits.
/// The array texture index is used to pick the right texture
/// in the shader. It's essentially the texture id.
fn pack_texture_data(uvs: [[f32; 2]; 4], tex_id: u32) -> [u32; 4] {
    let mut data: [u32; 4] = [0; 4];
    for (i, uv) in uvs.iter().enumerate() {
        let u = (uv[0] as u32 & 0x1) << 12;
        let v = (uv[1] as u32 & 0x1) << 13;
        data[i] = tex_id & 0xFFF | u | v;
    }
    data
}
