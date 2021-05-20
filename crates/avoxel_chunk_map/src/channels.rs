use avoxel_chunk::{Chunk, Lz4CompressedChunk};
use crossbeam_channel::{unbounded, Receiver, Sender};
use parking_lot::Mutex;
use std::{sync::Arc, time::Instant};

pub struct ChunkGenChannels {
    /// Sending Instant for timing purposes
    pub(crate) tx: Sender<(Chunk, Instant)>,
    pub(crate) rx: Receiver<(Chunk, Instant)>,
}

impl Default for ChunkGenChannels {
    fn default() -> Self {
        let (tx, rx) = unbounded();
        Self { tx, rx }
    }
}

pub struct DecompressionChannels {
    pub(crate) tx: Sender<Arc<Mutex<Chunk>>>,
    pub(crate) rx: Receiver<Arc<Mutex<Chunk>>>,
}

impl Default for DecompressionChannels {
    fn default() -> Self {
        let (tx, rx) = unbounded();
        Self { tx, rx }
    }
}

pub struct CompressionChannels {
    /// Sending Instant for timing purposes
    pub(crate) tx: Sender<(Lz4CompressedChunk, Instant)>,
    pub(crate) rx: Receiver<(Lz4CompressedChunk, Instant)>,
}

impl Default for CompressionChannels {
    fn default() -> Self {
        let (tx, rx) = unbounded();
        Self { tx, rx }
    }
}
