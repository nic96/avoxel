use serde::{Deserialize, Serialize};

/// In the future this struct will contain more fields for texture
/// For example texture biome hue to change colors base on biomes
/// And maybe even season hues/color offsets for seasons that would be passed to the shader.
/// Could also store UV coordinates in the block texture instead of calculating them
/// for every block during meshing. Random rotations, would probably still be calculated
/// by the mesher.
#[derive(Serialize, Deserialize, Copy, Clone, Default)]
pub struct BlockTexture {
    /// Defines whether the texture should be randomly rotated
    pub rand_rot: bool,
}
