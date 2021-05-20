use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Block {
    /// The order of texture ids is top, right, bottom, left, front, back.
    /// The texture ids are used to calculate uv coordinates for blocks.
    pub texture_ids: [u32; 6],
    pub name: String,
    /// Does the block contribute to ao (ambient occlusion)
    pub ao: bool,
    /// Is the block transparent
    pub transparent: bool,
}

impl Block {
    /// The block id for air
    pub const AIR: u32 = 0;
    pub const TOP: usize = 0;
    pub const RIGHT: usize = 1;
    pub const BOTTOM: usize = 2;
    pub const LEFT: usize = 3;
    pub const FRONT: usize = 4;
    pub const BACK: usize = 5;
}
