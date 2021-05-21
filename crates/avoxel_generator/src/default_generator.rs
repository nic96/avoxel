use avoxel_chunk::Chunk;
use avoxel_math::Pos;
use noise::{NoiseFn, Perlin, Seedable};

pub fn generate_chunk(pos: &Pos) -> Chunk {
    let noise_factor = 20.0;
    let noise_scale = 0.02;
    let mut chunk = Chunk::new(*pos, 0);

    let mut extent = chunk.extent();
    // Extent max is inclusive, but range max is exclusive
    extent.max += 1;
    if extent.min.y > noise_factor as i32 {
        return chunk;
    }
    if extent.max.y < -noise_factor as i32 {
        chunk = Chunk::new(*pos, 1);
        return chunk;
    }

    let perlin = Perlin::new();
    perlin.set_seed(1);
    for z in extent.min.z..extent.max.z {
        for x in extent.min.x..extent.max.x {
            // height
            let h = perlin.get([x as f64 * noise_scale, z as f64 * noise_scale]);
            let mut h = (noise_factor * h).floor() as i32;
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
