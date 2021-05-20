use avoxel_chunk::CHUNK_SIZE;
use avoxel_math::{BevyVec3, Pos};
use bevy::ecs::event::Events;
use bevy::prelude::{Query, ResMut, Vec3};
use indexmap::set::IndexSet;

#[derive(Default)]
pub struct ChunkViewerMoveEvent;

/// Viewer entities are what causes chunks to get generated or loaded.
/// Chunks load, generate, and render around a viewer.
pub struct ChunkViewer {
    pos: Pos,
    render_distance: i32,
    visible_chunks: IndexSet<Pos>,
    moved: bool,
}

impl ChunkViewer {
    pub fn new(render_distance: i32, pos: Pos) -> Self {
        let mut chunk_viewer = Self {
            pos,
            render_distance,
            visible_chunks: IndexSet::new(),
            moved: true,
        };
        chunk_viewer.update_visible_chunks();
        chunk_viewer
    }

    pub(crate) fn get_visible_chunks(&self) -> &IndexSet<Pos> {
        &self.visible_chunks
    }

    pub fn set_translation(&mut self, translation: Vec3) {
        let chunk_pos = translation / CHUNK_SIZE as f32;
        let prev_pos = self.pos;
        self.pos = Pos::from_vec3(&chunk_pos);
        if prev_pos != self.pos {
            self.moved = true;
            self.update_visible_chunks();
        }
    }

    fn update_visible_chunks(&mut self) {
        // reset visible chunks
        self.visible_chunks = IndexSet::new();

        let start_x = self.pos.x - self.render_distance;
        let start_z = self.pos.z - self.render_distance;
        let start_y = self.pos.y - self.render_distance;
        let end_x = self.pos.x + self.render_distance;
        let end_z = self.pos.z + self.render_distance;
        let end_y = self.pos.y + self.render_distance;

        for x in start_x..end_x {
            for z in start_z..end_z {
                for y in (start_y..end_y).rev() {
                    self.visible_chunks.insert(Pos::new(x, y, z));
                }
            }
        }
    }

    pub fn get_pos(&self) -> Pos {
        self.pos
    }
}

pub(crate) fn chunk_viewer_moved(
    mut events: ResMut<Events<ChunkViewerMoveEvent>>,
    mut viewers: Query<&mut ChunkViewer>,
) {
    let mut moved = false;
    for mut viewer in viewers.iter_mut() {
        if viewer.moved {
            moved = true;
            viewer.moved = false;
        }
    }
    if moved {
        events.send(ChunkViewerMoveEvent);
    }
}
