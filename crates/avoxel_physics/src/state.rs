#[derive(PartialEq)]
pub enum AvoxelPhysicsStates {
    Active,
    Paused,
}

pub struct AvoxelPhysicsState {
    state: AvoxelPhysicsStates,
}

impl Default for AvoxelPhysicsState {
    fn default() -> Self {
        Self {
            state: AvoxelPhysicsStates::Active,
        }
    }
}

impl AvoxelPhysicsState {
    pub fn new(state: AvoxelPhysicsStates) -> Self {
        Self { state }
    }

    pub fn pause(&mut self) {
        self.state = AvoxelPhysicsStates::Paused;
    }

    pub fn resume(&mut self) {
        self.state = AvoxelPhysicsStates::Active;
    }

    pub fn paused(&self) -> bool {
        self.state == AvoxelPhysicsStates::Paused
    }

    pub fn active(&self) -> bool {
        self.state == AvoxelPhysicsStates::Active
    }
}
