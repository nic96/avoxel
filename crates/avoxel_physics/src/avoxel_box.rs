use avoxel_math::{Aabb, BevyVec3, Pos};
use bevy::prelude::Vec3;

pub struct AvoxelBox {
    /// The bounding box
    aabb: Aabb,
    /// The current position
    pub(crate) translation: Vec3,
    pub(crate) on_floor: bool,
    pub(crate) on_ceiling: bool,
    pub(crate) velocity: Vec3,
    pub(crate) changes: AvoxelBoxChanges,
}

impl AvoxelBox {
    pub fn new(translation: Vec3, aabb: Aabb) -> Self {
        Self {
            aabb,
            translation,
            on_floor: false,
            on_ceiling: false,
            velocity: Vec3::ZERO,
            changes: AvoxelBoxChanges::all(),
        }
    }

    /// Adds the given impulse vector to the current velocity
    pub fn impulse(&mut self, impulse_vector: Vec3) {
        self.velocity += impulse_vector;
    }

    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Vec3) {
        self.velocity = velocity;
    }

    pub fn clamp_velocity(&mut self, max_velocity: f32) {
        if self.velocity.length_squared() > max_velocity * max_velocity {
            self.velocity = self.velocity.normalize() * max_velocity;
        }
    }

    /// only valid after calling move_and_slide
    pub fn is_on_floor(&self) -> bool {
        self.on_floor
    }

    pub fn translated_aabb(&self) -> Aabb {
        Aabb {
            min: self.aabb.min + self.translation,
            max: self.aabb.max + self.translation,
        }
    }

    pub fn pos(&self) -> Pos {
        Pos::from_vec3(&self.translated_aabb().min.floor())
    }
}

pub struct AvoxelBoxBuilder {
    pub(crate) aabb: Aabb,
    pub(crate) translation: Vec3,
}

impl AvoxelBoxBuilder {
    pub fn new(translation: Vec3, aabb: Aabb) -> AvoxelBoxBuilder {
        Self { aabb, translation }
    }

    pub(crate) fn build(&self) -> AvoxelBox {
        AvoxelBox::new(self.translation, self.aabb)
    }
}

bitflags::bitflags! {
    pub(crate) struct AvoxelBoxChanges: u32 {
        const MODIFIED = 1 << 0;
        const POSITION  = 1 << 1;
    }
}
