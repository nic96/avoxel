use avoxel_blocks::BlockLibrary;
use bevy::prelude::*;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum States {
    LoadingMaterials,
    Meshing,
}

pub struct State {
    active_state: States,
}

impl Default for State {
    fn default() -> State {
        State {
            active_state: States::LoadingMaterials,
        }
    }
}

impl State {
    pub fn set_state(&mut self, state: States) {
        self.active_state = state;
    }

    pub fn get_state(&self) -> States {
        self.active_state
    }
}

pub fn handle_state_system(
    mut state: ResMut<State>,
    block_library: Res<BlockLibrary>,
    mut textures: ResMut<Assets<Texture>>,
) {
    match state.get_state() {
        States::LoadingMaterials => {
            let mut loaded = true;
            if let Some(texture) = textures.get_mut(&block_library.get_texture_handle()) {
                texture.reinterpret_stacked_2d_as_array(block_library.get_texture_count());
            } else {
                loaded = false;
            }
            if loaded {
                state.set_state(States::Meshing);
                info!("Loaded block textures");
            }
        }
        States::Meshing => {}
    }
}
