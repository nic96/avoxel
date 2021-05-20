use avoxel::blocks::{Block, BlockLibrary, BlockMaterial, BlockTexture};
use bevy::prelude::*;

pub struct BlockLibraryPlugin;

impl Plugin for BlockLibraryPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // I'll probably come up with a better block library system
        // or come up with a completely different idea
        let mut block_library = BlockLibrary::new();
        block_library
            // 0 grass
            .add_block_texture(BlockTexture { rand_rot: true })
            // 1 grass_side
            .add_block_texture(BlockTexture { rand_rot: false })
            // 2 dirt
            .add_block_texture(BlockTexture { rand_rot: true })
            // block id 0, this block will never get rendered
            // and is here just to take up index 0 in this vec
            // and is required to be here. Do not remove it.
            .add_block(Block {
                texture_ids: [0; 6],
                name: "air".to_string(),
                ao: false,
                transparent: false,
            })
            // block id 1
            .add_block(Block {
                name: "grass".to_string(),
                // the order is top, right, bottom, left, front, back
                texture_ids: [0, 1, 2, 1, 1, 1],
                ao: true,
                transparent: false,
            })
            // block id 2
            .add_block(Block {
                name: "dirt".to_string(),
                // the order is top, right, bottom, left, front, back
                texture_ids: [2; 6],
                ao: true,
                transparent: false,
            });

        app.insert_resource(block_library)
            .add_startup_system(setup_materials.system());
    }
}

fn setup_materials(
    mut block_library: ResMut<BlockLibrary>,
    mut materials: ResMut<Assets<BlockMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("textures/block_textures.png");
    block_library
        .set_texture_path("textures/block_textures.png")
        .set_texture_handle(texture_handle)
        .set_texture_size(16)
        .set_texture_count(4);

    let texture_handle = block_library.get_texture_handle();
    block_library.add_material_handle(materials.add(BlockMaterial {
        base_color: Color::rgb(1.0, 1.0, 1.0),
        base_color_texture: Some(texture_handle),
        roughness: 0.8,
        reflectance: 0.1,
        ..Default::default()
    }));
}
