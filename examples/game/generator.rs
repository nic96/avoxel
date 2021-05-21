use avoxel::chunk::Chunk;
use avoxel::math::Pos;
use noise::{NoiseFn, Perlin, Seedable};

const NOISE_FACTOR: f32 = 10.0;
const NOISE_SCALE: f64 = 0.04;

pub fn generate_chunk(pos: &Pos) -> Chunk {
    let mut chunk = Chunk::new(*pos, 0);

    let mut extent = chunk.extent();
    // Extent max is inclusive, but range max is exclusive
    extent.max += 1;
    if extent.min.y > NOISE_FACTOR as i32 {
        return chunk;
    }
    if extent.max.y < -NOISE_FACTOR as i32 {
        chunk = Chunk::new(*pos, 1);
        return chunk;
    }

    let perlin = Perlin::new();
    perlin.set_seed(1);
    for z in extent.min.z..extent.max.z {
        for x in extent.min.x..extent.max.x {
            // height
            let h = perlin.get([x as f64 * NOISE_SCALE, z as f64 * NOISE_SCALE]) as f32;
            let mut h = (NOISE_FACTOR * h).floor() as i32;
            if h > extent.min.y {
                if h > extent.max.y {
                    h = extent.max.y;
                }
                chunk.fill_area(1, Pos::new(x, extent.min.y, z), Pos::new(x + 1, h, z + 1));
            }
        }
    }
    chunk
}
