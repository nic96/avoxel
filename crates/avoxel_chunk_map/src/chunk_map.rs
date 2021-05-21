use crate::{
    channels::{ChunkGenChannels, CompressionChannels, DecompressionChannels},
    tools,
    tools::VoxelRayCastResult,
};
use avoxel_blocks::{Block, BlockLibrary};
use avoxel_chunk::{Chunk, Lz4CompressedChunk, Voxel, CHUNK_SIZE};
use avoxel_generator::default_generator;
use avoxel_math::{DivFloor, Pos};
use bevy::{
    prelude::*,
    tasks::AsyncComputeTaskPool,
    utils::{HashMap, HashSet},
};
use indexmap::set::IndexSet;
use parking_lot::Mutex;
use std::{collections::hash_map::Keys, iter::Chain, sync::Arc, time::Instant};

pub struct ChunkMap {
    /// The storage for `Chunks`. A chunk doesn't need to be accessed by more
    /// then one thread at once, but we do need to sometimes modify the `chunk` in
    /// another thread therefore the `Mutex`.
    pub chunks: HashMap<Pos, Arc<Mutex<Chunk>>>,
    /// Compressed chunks
    pub compressed_chunks: HashMap<Pos, Lz4CompressedChunk>,
    pub(crate) compression_level: u32,
    /// Whether to use LittleEndian byteorder when compressing voxels or native byteorder
    pub(crate) compress_byteorder: bool,
    /// Visible chunks contains coordinates for chunks that should be loaded
    pub visible_chunks: HashSet<Pos>,
    /// Dirty chunks are chunks that need to be re-meshed
    pub dirty_chunks: HashSet<Pos>,
    /// Chunks that are currently being generated or loaded in other threads
    /// Needed so we don't load the same chunk twice
    loading_chunks: IndexSet<Pos>,
    /// Channels to send and receive generated chunks
    pub(crate) gen_channels: ChunkGenChannels,
    pub(crate) compression_channels: CompressionChannels,
    pub(crate) decompression_channels: DecompressionChannels,
    /// Block Library used by mesher for meshing and texturing
    pub block_library: Arc<BlockLibrary>,
    pub generator: &'static (dyn Fn(&Pos) -> Chunk + Send + Sync + 'static),
}

impl Default for ChunkMap {
    fn default() -> Self {
        Self {
            chunks: Default::default(),
            compressed_chunks: Default::default(),
            compression_level: 10,
            compress_byteorder: false,
            visible_chunks: Default::default(),
            dirty_chunks: Default::default(),
            loading_chunks: Default::default(),
            gen_channels: Default::default(),
            compression_channels: Default::default(),
            decompression_channels: Default::default(),
            block_library: Arc::new(Default::default()),
            generator: &default_generator::generate_chunk,
        }
    }
}

pub enum ChunkState {
    Unloaded,
    /// Can be either loading or generating
    Loading,
    /// Chunk needs to be re-meshed
    Dirty,
    /// Loaded
    Loaded,
}

impl ChunkMap {
    /// * `byteorder` - if set to true uses LittleEndian byteorder when compressing instead of native byteorder
    pub fn new(
        byteorder: bool,
        generator: &'static (dyn Fn(&Pos) -> Chunk + Send + Sync + 'static),
    ) -> Self {
        Self {
            compress_byteorder: byteorder,
            generator,
            ..Default::default()
        }
    }

    pub fn set_byteorder(&mut self, byteorder: bool) {
        assert_eq!(self.compressed_chunks.len(), 0);
        self.compress_byteorder = byteorder;
    }

    pub fn get_byteorder(&self) -> bool {
        self.compress_byteorder
    }

    pub fn contains_chunk(&self, pos: &Pos) -> bool {
        self.chunks.contains_key(pos) || self.compressed_chunks.contains_key(pos)
    }

    pub fn chunk_keys(&self) -> ChunkKeys {
        self.chunks.keys().chain(self.compressed_chunks.keys())
    }

    pub(crate) fn chunk_state(&self, pos: &Pos) -> ChunkState {
        if self.contains_chunk(pos) {
            ChunkState::Loaded
        } else if self.loading_chunks.contains(pos) {
            ChunkState::Loading
        } else if self.dirty_chunks.contains(pos) {
            ChunkState::Dirty
        } else {
            ChunkState::Unloaded
        }
    }

    /// Returns the decompressed chunk if it existed in the compressed chunks.
    /// The chunk is decompressed then moved into the chunks HashMap before being returned.
    fn decompress_chunk(&mut self, pos: &Pos) -> Option<Arc<Mutex<Chunk>>> {
        if let Some(compressed_chunk) = self.compressed_chunks.remove(pos) {
            let chunk = Arc::new(Mutex::new(
                compressed_chunk.decompress(self.compress_byteorder),
            ));
            self.chunks.insert(*pos, chunk.clone());
            return Some(chunk);
        }
        None
    }

    /// Multiple chunks can contain the same position due to padding so if you plan
    /// on modify the chunks make sure you also modify the neighboring chunks if any
    fn get_mut_chunks_containing_pos(&mut self, pos: &Pos) -> Vec<Arc<Mutex<Chunk>>> {
        let mut chunk_keys = vec![];
        let chunk_key = pos.div_floor(CHUNK_SIZE);
        chunk_keys.push(chunk_key);
        let x_rem = pos.x % CHUNK_SIZE;
        if x_rem == 0 {
            chunk_keys.push(chunk_key + Pos::new(-1, 0, 0));
        } else if x_rem == 63 || x_rem == -1 {
            chunk_keys.push(chunk_key + Pos::new(1, 0, 0));
        }
        let y_rem = pos.y % CHUNK_SIZE;
        if y_rem == 0 {
            chunk_keys.push(chunk_key + Pos::new(0, -1, 0));
        } else if y_rem == 63 || y_rem == -1 {
            chunk_keys.push(chunk_key + Pos::new(0, 1, 0));
        }
        let z_rem = pos.z % CHUNK_SIZE;
        if z_rem == 0 {
            chunk_keys.push(chunk_key + Pos::new(0, 0, -1));
        } else if z_rem == 63 || z_rem == -1 {
            chunk_keys.push(chunk_key + Pos::new(0, 0, 1));
        }
        let mut chunks = vec![];
        for chunk_key in &chunk_keys {
            if let Some(chunk) = self.chunks.get(chunk_key) {
                chunks.push(chunk.clone());
            } else if let Some(chunk) = self.decompress_chunk(&chunk_key) {
                chunks.push(chunk.clone());
            };
        }
        chunks
    }

    /// It's best to not mutate the chunk if using the method since if the chunk was compressed
    /// it will be sent via a channel and if the send fails the mutation won't be applied.
    fn get_chunk_containing_pos(&self, pos: &Pos) -> Option<Arc<Mutex<Chunk>>> {
        let chunk_key = pos.div_floor(CHUNK_SIZE);
        if let Some(chunk) = self.chunks.get(&chunk_key) {
            return Some(chunk.clone());
        };

        // check if chunk is in channel resending chunks retrieved from channel
        let mut decompressed_chunks = vec![];
        let mut return_chunk = None;
        for chunk in self.decompression_channels.rx.clone().try_iter() {
            decompressed_chunks.push(chunk.clone());
            if chunk.lock().pos == chunk_key {
                return_chunk = Some(chunk);
                break;
            }
        }
        for chunk in decompressed_chunks {
            match self.decompression_channels.tx.send(chunk) {
                Ok(_) => {}
                Err(e) => {
                    bevy::log::warn!(
                        "failed to send chunk over decompression channel: {:#?}",
                        e.0.lock().pos
                    )
                }
            }
        }
        if return_chunk.is_some() {
            return return_chunk;
        }

        // check if chunk is in compressed_chunks.
        // decompress, send in channel, and return chunk if it is.
        return if let Some(compressed_chunk) = self.compressed_chunks.get(&chunk_key) {
            let chunk = Arc::new(Mutex::new(
                compressed_chunk.decompress(self.compress_byteorder),
            ));
            // Sends the chunk over the decompression channels.
            // The chunk will be received by `store_decompressed_chunks` system and stored
            match self.decompression_channels.tx.send(chunk.clone()) {
                Ok(_) => {}
                Err(e) => {
                    bevy::log::warn!(
                        "failed to send chunk over decompression channel: {:#?}",
                        e.0.lock().pos
                    )
                }
            }
            Some(chunk)
        } else {
            None
        };
    }

    pub fn get_voxel(&self, pos: &Pos) -> Option<Voxel> {
        return if let Some(chunk) = self.get_chunk_containing_pos(&pos) {
            Some(chunk.lock().get_voxel(*pos))
        } else {
            None
        };
    }

    pub fn make_dirty(&mut self, pos: &Pos) {
        self.dirty_chunks.insert(*pos);
    }

    pub fn ray_cast(
        &self,
        origin: Vec3,
        direction: Vec3,
        max_d: f32,
    ) -> Option<VoxelRayCastResult> {
        let is_hit = |pos: Pos| -> bool {
            if let Some(v) = self.get_voxel(&pos) {
                return v != Block::AIR;
            }
            // if failed to get_voxel return
            // we want to cancel ray casting cause this shouldn't happen
            true
        };
        tools::voxel_ray_cast(is_hit, origin, direction, max_d)
    }

    pub(crate) fn remove_chunk(&mut self, pos: &Pos) {
        if self.chunks.remove(pos).is_none() {
            self.compressed_chunks.remove(pos);
        }
    }

    pub fn set_voxel(&mut self, voxel: Voxel, pos: &Pos) {
        for chunk in self.get_mut_chunks_containing_pos(&pos) {
            let mut chunk = chunk.lock();
            chunk.set_voxel(voxel, *pos);
            if cfg!(feature = "mesher") {
                self.make_dirty(&chunk.pos);
            }
        }
    }

    pub(crate) fn set_chunk_state_loading(&mut self, pos: &Pos) {
        self.loading_chunks.insert(*pos);
    }

    pub(crate) fn set_chunk_state_loaded(&mut self, pos: &Pos) {
        self.loading_chunks.remove(pos);
    }

    pub fn compress_chunk(&self, pool: AsyncComputeTaskPool, pos: Pos) {
        let sender = self.compression_channels.tx.clone();
        let compression_level = self.compression_level;
        if let Some(chunk) = self.chunks.get(&pos) {
            let chunk = chunk.clone();
            pool.spawn(async move {
                let chunk = chunk.lock();
                let start_instant = Instant::now();
                match sender.send((chunk.compress(compression_level, false), start_instant)) {
                    Ok(_) => {}
                    Err(_e) => {
                        warn!("failed to send compressed chunk with channel");
                    }
                }
            })
            .detach();
        }
    }

    pub fn clear_channels(&self) {
        for _ in self.decompression_channels.rx.clone().try_iter() {}
        for _ in self.compression_channels.rx.clone().try_iter() {}
        for _ in self.gen_channels.rx.clone().try_iter() {}
    }
}

type ChunkKeys<'a> = Chain<Keys<'a, Pos, Arc<Mutex<Chunk>>>, Keys<'a, Pos, Lz4CompressedChunk>>;
