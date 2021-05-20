use crate::block::Block;
use crate::block_texture::BlockTexture;
use avoxel_rendering::prelude::BlockMaterial;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockLibrary {
    blocks: Vec<Block>,
    textures: Vec<BlockTexture>,
    texture_path: Option<String>,
    /// The size of the individual textures on the texture atlas
    texture_size: u32,
    /// The number of textures on the texture atlas
    texture_count: u32,
    #[serde(skip)]
    texture_handle: Handle<Texture>,
    #[serde(skip)]
    material_handles: Vec<Handle<BlockMaterial>>,
}

impl Default for BlockLibrary {
    fn default() -> Self {
        Self {
            // we initialize it with a bunch of blocks
            // by default in case the generator needs those
            blocks: vec![Block::default(); 1024],
            textures: vec![BlockTexture::default()],
            texture_path: None,
            texture_size: 32,
            texture_count: 1024,
            texture_handle: Default::default(),
            material_handles: vec![Handle::default()],
        }
    }
}

impl BlockLibrary {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            textures: vec![],
            texture_path: None,
            texture_size: 32,
            texture_count: 1024,
            texture_handle: Default::default(),
            material_handles: vec![],
        }
    }

    pub fn get_block(&self, block_id: usize) -> &Block {
        &self.blocks[block_id]
    }

    pub fn add_block(&mut self, block: Block) -> &mut Self {
        self.blocks.push(block);
        self
    }

    pub fn add_block_texture(&mut self, block_texture: BlockTexture) -> &mut Self {
        self.textures.push(block_texture);
        self
    }

    pub fn get_block_texture(&self, texture_id: u32) -> BlockTexture {
        self.textures[texture_id as usize]
    }

    pub fn get_block_library_as_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    pub fn set_texture_path(&mut self, path: &str) -> &mut Self {
        self.texture_path = Some(path.to_string());
        self
    }

    pub fn get_texture_path(&self) -> Option<String> {
        self.texture_path.clone()
    }

    pub fn set_texture_size(&mut self, size: u32) -> &mut Self {
        self.texture_size = size;
        self
    }

    pub fn get_texture_size(self) -> u32 {
        self.texture_size
    }

    pub fn set_texture_count(&mut self, count: u32) -> &mut Self {
        self.texture_count = count;
        self
    }

    pub fn get_texture_count(&self) -> u32 {
        self.texture_count
    }

    pub fn get_texture_handle(&self) -> Handle<Texture> {
        self.texture_handle.clone()
    }

    pub fn set_texture_handle(&mut self, texture_handle: Handle<Texture>) -> &mut Self {
        self.texture_handle = texture_handle;
        self
    }

    pub fn add_material_handle(&mut self, material_handle: Handle<BlockMaterial>) -> &mut Self {
        self.material_handles.push(material_handle);
        self
    }

    pub fn get_material_handle(&self, material_id: usize) -> Handle<BlockMaterial> {
        self.material_handles[material_id].clone()
    }
}
