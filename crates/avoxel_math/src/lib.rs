mod aabb;

use vek::Vec3;

pub type Pos = Vec3<i32>;
pub type Extent3 = vek::Aabb<i32>;
pub use aabb::Aabb;

impl BevyVec3 for Pos {
    fn to_vec3(&self) -> bevy_math::Vec3 {
        bevy_math::Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    fn from_vec3(vec3: &bevy_math::Vec3) -> Pos {
        Pos::new(vec3.x as i32, vec3.y as i32, vec3.z as i32)
    }
}

impl DivFloor<i32> for Pos {
    fn div_floor(&self, rhs: i32) -> Self {
        // Algorithm from [Daan Leijen. _Division and Modulus for Computer Scientists_,
        // December 2001](http://research.microsoft.com/pubs/151917/divmodnote-letter.pdf)
        let (d, r) = (self.x / rhs, self.x % rhs);
        let x = if (r > 0 && rhs < 0) || (r < 0 && rhs > 0) {
            d - 1
        } else {
            d
        };
        let (d, r) = (self.y / rhs, self.y % rhs);
        let y = if (r > 0 && rhs < 0) || (r < 0 && rhs > 0) {
            d - 1
        } else {
            d
        };
        let (d, r) = (self.z / rhs, self.z % rhs);
        let z = if (r > 0 && rhs < 0) || (r < 0 && rhs > 0) {
            d - 1
        } else {
            d
        };
        Pos::new(x, y, z)
    }
}

pub trait ToTuple {
    fn to_tuple(&self) -> (i32, i32, i32);
}

pub trait BevyVec3 {
    fn to_vec3(&self) -> bevy_math::Vec3;

    fn from_vec3(vec3: &bevy_math::Vec3) -> Self;
}

pub trait DivFloor<Rhs = Self> {
    fn div_floor(&self, rhs: Rhs) -> Self;
}
