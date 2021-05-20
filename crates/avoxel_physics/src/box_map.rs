use crate::avoxel_box::AvoxelBox;
use generational_arena::Arena;

pub type AvoxelBoxHandle = generational_arena::Index;
/// Storage for AvoxelBoxes to be handled by physics systems
#[derive(Default)]
pub struct BoxMap {
    pub(crate) boxes: Arena<AvoxelBox>,
}

impl BoxMap {
    pub(crate) fn insert(&mut self, a_box: AvoxelBox) -> AvoxelBoxHandle {
        self.boxes.insert(a_box)
    }

    pub fn get_mut(&mut self, handle: AvoxelBoxHandle) -> Option<&mut AvoxelBox> {
        self.boxes.get_mut(handle)
    }

    pub fn get(&self, handle: AvoxelBoxHandle) -> Option<&AvoxelBox> {
        self.boxes.get(handle)
    }
}
