use crate::{
    chunk::{Chunk, CHUNK_PADDING, CHUNK_SIZE, CHUNK_STORAGE_SIZE},
    voxel::Voxel,
};
use avoxel_math::{Extent3, Pos};
use byteorder::{ByteOrder, LittleEndian};

/// A compressed `Chunk`
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Lz4CompressedChunk {
    #[serde(with = "serde_bytes")]
    pub compressed_voxels: Vec<u8>,
    pub pos: Pos,
    pub ambient_voxel: Voxel,
    pub empty: bool,
}

impl Lz4CompressedChunk {
    pub fn extent(&self) -> Extent3 {
        Extent3 {
            min: self.pos * CHUNK_SIZE - CHUNK_PADDING,
            max: self.pos * CHUNK_SIZE + CHUNK_SIZE + CHUNK_PADDING - 1,
        }
    }
}

impl Lz4CompressedChunk {
    pub fn decompress(&self, byteorder: bool) -> Chunk {
        let num_points;
        if self.empty {
            num_points = 0;
        } else {
            num_points = CHUNK_STORAGE_SIZE;
        }

        let mut decoder = lz4::Decoder::new(self.compressed_voxels.as_slice()).unwrap();

        let mut decompressed_voxels: Vec<Voxel> = Vec::with_capacity(num_points);
        unsafe { decompressed_voxels.set_len(num_points) };
        if byteorder {
            let mut bytes: Vec<u8> = vec![0; num_points * core::mem::size_of::<Voxel>()];
            std::io::copy(&mut decoder, &mut bytes.as_mut_slice()).unwrap();
            LittleEndian::read_u32_into(bytes.as_slice(), &mut decompressed_voxels.as_mut_slice());
        } else {
            let mut decompressed_slice = unsafe {
                std::slice::from_raw_parts_mut(
                    decompressed_voxels.as_mut_ptr() as *mut u8,
                    num_points * core::mem::size_of::<Voxel>(),
                )
            };
            std::io::copy(&mut decoder, &mut decompressed_slice).unwrap();
        }

        Chunk::new_from_vec(self.pos, self.ambient_voxel, decompressed_voxels)
    }
}
