use crate::{compressed_chunk::Lz4CompressedChunk, voxel::Voxel};
use avoxel_math::{Extent3, Pos};
use bevy_math::Vec3;
use byteorder::{ByteOrder, LittleEndian};

// Chunk padding used so neighbor chunk lookups aren't needed in a lot of cases
pub const CHUNK_PADDING: i32 = 1;
pub const CHUNK_SIZE: i32 = 64;
pub const CHUNK_SIZE_WITH_PADDING: i32 = CHUNK_PADDING * 2 + CHUNK_SIZE;
pub const CHUNK_LAYER_SIZE_WITH_PADDING: i32 = CHUNK_SIZE_WITH_PADDING * CHUNK_SIZE_WITH_PADDING;
pub const CHUNK_STORAGE_SIZE: usize =
    (CHUNK_SIZE_WITH_PADDING * CHUNK_SIZE_WITH_PADDING * CHUNK_SIZE_WITH_PADDING) as usize;

pub struct ChunkTag;

#[derive(Clone)]
pub struct Chunk {
    /// And array of voxels. Each voxel is simply a u32
    pub voxels: Vec<Voxel>,
    pub ambient_voxel: Voxel,
    /// World position divided by chunk size
    pub pos: Pos,
}

impl Chunk {
    pub fn new(pos: Pos, initial_voxel: Voxel) -> Chunk {
        Chunk {
            pos,
            ambient_voxel: initial_voxel,
            voxels: vec![],
        }
    }

    pub fn new_from_vec(pos: Pos, ambient_voxel: Voxel, voxels: Vec<Voxel>) -> Chunk {
        Chunk {
            pos,
            ambient_voxel,
            voxels,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.voxels.len() == 0
    }

    fn fill_if_empty(&mut self) {
        if self.is_empty() {
            self.voxels = vec![self.ambient_voxel; CHUNK_STORAGE_SIZE as usize];
        }
    }

    /// Uses unstable feature slice_fill to fill an area with voxels of a certain type
    pub fn fill_area(&mut self, voxel: u32, min: Pos, max: Pos) {
        self.fill_if_empty();
        for z in min.z..max.z {
            for x in min.x..max.x {
                let start_i = self.block_index(Pos::new(x, min.y, z));
                let end_i = self.block_index(Pos::new(x, max.y, z));
                self.voxels[start_i..end_i].fill(voxel);
            }
        }
    }

    pub fn block_index(&self, pos: Pos) -> usize {
        let local_pos = pos - self.extent().min;
        (local_pos.y
            + (local_pos.z * CHUNK_SIZE_WITH_PADDING)
            + (local_pos.x * CHUNK_LAYER_SIZE_WITH_PADDING)) as usize
    }

    pub fn index_to_pos(&self, i: usize) -> Pos {
        let mut i = i as i32;
        let x = i / CHUNK_LAYER_SIZE_WITH_PADDING;
        i -= x * CHUNK_LAYER_SIZE_WITH_PADDING;
        let z = i / CHUNK_SIZE_WITH_PADDING;
        let y = i % CHUNK_SIZE_WITH_PADDING;

        Pos::new(x, y, z) + self.extent().min
    }

    /// When using this method make make sure the position is within the bounds of the chunk
    pub fn get_voxel(&self, pos: Pos) -> u32 {
        if self.is_empty() {
            return self.ambient_voxel;
        }
        self.voxels[self.block_index(pos)]
    }

    /// When using this method make sure the position is within the bounds of the chunk
    pub fn set_voxel(&mut self, voxel: u32, pos: Pos) {
        self.fill_if_empty();
        let i = self.block_index(pos);
        self.voxels[i] = voxel;
    }

    pub fn get_chunk_translation(self) -> Vec3 {
        let p = self.pos * Pos::new(CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE);
        Vec3::new(p.x as f32, p.y as f32, p.z as f32)
    }

    pub fn get_voxel_translation(self, pos: Pos) -> Vec3 {
        self.get_chunk_translation() + Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32)
    }

    pub fn extent(&self) -> Extent3 {
        Extent3 {
            min: self.pos * CHUNK_SIZE - CHUNK_PADDING,
            max: self.pos * CHUNK_SIZE + CHUNK_SIZE + CHUNK_PADDING - 1,
        }
    }

    // Compress the map in-memory using the LZ4 algorithm.
    //
    // WARNING: If byteorder = false, the voxels vec will be used as a byte slice without
    // accounting for endianness. This is not compatible across platforms.
    pub fn compress(&self, compression_level: u32, byteorder: bool) -> Lz4CompressedChunk {
        let mut compressed_bytes = Vec::new();
        let mut encoder = lz4::EncoderBuilder::new()
            .level(compression_level)
            .build(&mut compressed_bytes)
            .unwrap();

        if byteorder {
            let mut bytes = vec![0; self.voxels.len() * core::mem::size_of::<Voxel>()];
            LittleEndian::write_u32_into(&self.voxels, bytes.as_mut_slice());
            std::io::copy(&mut std::io::Cursor::new(bytes), &mut encoder).unwrap();
        } else {
            let values_slice: &[u8] = unsafe {
                std::slice::from_raw_parts(
                    self.voxels.as_ptr() as *const u8,
                    self.voxels.len() * core::mem::size_of::<Voxel>(),
                )
            };
            std::io::copy(&mut std::io::Cursor::new(values_slice), &mut encoder).unwrap();
        }

        let (_output, _result) = encoder.finish();

        Lz4CompressedChunk {
            pos: self.pos,
            ambient_voxel: self.ambient_voxel,
            compressed_voxels: compressed_bytes,
            empty: self.is_empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        chunk::{Chunk, CHUNK_STORAGE_SIZE},
        voxel::Voxel,
    };
    use avoxel_math::Pos;

    #[test]
    fn compression() {
        let mut voxels: Vec<Voxel> = vec![0; CHUNK_STORAGE_SIZE];
        for (i, v) in voxels.iter_mut().enumerate() {
            *v = (i % 24) as u32;
        }
        let chunk = Chunk::new_from_vec(Pos::new(0, 0, 0), 0, voxels);
        assert_eq!(
            chunk.compress(10, true).decompress(true).voxels,
            chunk.voxels
        );
        assert_eq!(
            chunk.compress(10, false).decompress(false).voxels,
            chunk.voxels
        );
    }
}
