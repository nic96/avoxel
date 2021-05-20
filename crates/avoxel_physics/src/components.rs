use crate::box_map::AvoxelBoxHandle;

pub struct AvoxelBoxHandleComponent(AvoxelBoxHandle);

impl AvoxelBoxHandleComponent {
    pub fn handle(&self) -> AvoxelBoxHandle {
        self.0
    }
}

impl From<AvoxelBoxHandle> for AvoxelBoxHandleComponent {
    fn from(h: AvoxelBoxHandle) -> Self {
        Self(h)
    }
}
