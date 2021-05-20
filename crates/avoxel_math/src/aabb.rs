use bevy_math::Vec3;

#[derive(Default, Debug, Copy, Clone)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn width(&self) -> f32 {
        debug_assert!(self.min.x <= self.max.x);
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        debug_assert!(self.min.y <= self.max.y);
        self.max.y - self.min.y
    }

    pub fn depth(&self) -> f32 {
        debug_assert!(self.min.z <= self.max.z);
        self.max.z - self.min.z
    }

    pub fn minkowski_difference(&self, other: &Aabb) -> Aabb {
        let md_min = self.min - other.max;
        let md_max_x = md_min.x + self.width() + other.width();
        let md_max_y = md_min.y + self.height() + other.height();
        let md_max_z = md_min.z + self.depth() + other.depth();

        Aabb {
            min: md_min,
            max: Vec3::new(md_max_x, md_max_y, md_max_z),
        }
    }

    pub fn colliding(&self, other: &Aabb) -> bool {
        (self.min.x <= other.max.x && self.max.x >= other.min.x)
            && (self.min.y <= other.max.y && self.max.y >= other.min.y)
            && (self.min.z <= other.max.z && self.max.z >= other.min.z)
    }

    pub fn point_colliding(&self, point: &Vec3) -> bool {
        self.min.x <= point.x
            && self.max.x >= point.x
            && self.min.y <= point.y
            && self.max.y >= point.y
            && self.min.z <= point.z
            && self.max.z >= point.z
    }

    /// Returns a vector to the point on the bounding box that is
    /// closest to a given point inside the bounding box.
    /// The return value can be added to a point to move it outside of the aabb.
    /// IMPORTANT: Only use this function on points that are known to be inside the aabb.
    pub fn penetration_vector(&self, point: &Vec3) -> Vec3 {
        // only use this function on points that are known to be inside the aabb
        debug_assert!(self.point_colliding(point));
        let mut x = point.x - self.max.x;
        let mut y = point.y - self.max.y;
        let mut z = point.z - self.max.z;
        let x_ = point.x - self.min.x;
        if x.abs() > x_.abs() {
            x = x_;
        };
        let y_ = point.y - self.min.y;
        if y.abs() > y_.abs() {
            y = y_;
        };
        let z_ = point.z - self.min.z;
        if z.abs() > z_.abs() {
            z = z_;
        };

        if y.abs() <= x.abs() && y.abs() <= z.abs() {
            Vec3::new(0., y, 0.)
        } else if x.abs() <= y.abs() && x.abs() <= z.abs() {
            Vec3::new(x, 0., 0.)
        } else {
            Vec3::new(0., 0., z)
        }
    }

    pub fn get_swept_broad_phase_box(&self, linear_velocity: &Vec3) -> Aabb {
        let v = linear_velocity;
        let b = self;
        Aabb {
            min: Vec3::new(
                if v.x > 0. { b.min.x } else { b.min.x + v.x },
                if v.y > 0. { b.min.y } else { b.min.y + v.y },
                if v.z > 0. { b.min.z } else { b.min.z + v.z },
            ),
            max: Vec3::new(
                if v.x > 0. { b.max.x + v.x } else { b.max.x },
                if v.y > 0. { b.max.y + v.y } else { b.max.y },
                if v.z > 0. { b.max.z + v.z } else { b.max.z },
            ),
        }
    }

    /// returns the time and normal that a collision happened
    /// if there was no collision returns 1 and a zeroed Vec3
    pub fn swept_collision_time_and_normal(
        &self,
        other: &Aabb,
        linear_velocity: &Vec3,
    ) -> (f32, Vec3) {
        let mut normal = Vec3::splat(0.);

        // These values are the inverse time until it hits the other object on the axis.
        let x_inv_entry;
        let y_inv_entry;
        let z_inv_entry;
        let x_inv_exit;
        let y_inv_exit;
        let z_inv_exit;

        // find the distance between the objects on the near and far sides for x, y, and z.
        if linear_velocity.x > 0. {
            x_inv_entry = other.min.x - self.max.x;
            x_inv_exit = other.max.x - self.min.x;
        } else {
            x_inv_entry = other.max.x - self.min.x;
            x_inv_exit = other.min.x - self.max.x;
        }

        if linear_velocity.y >= 0. {
            y_inv_entry = other.min.y - self.max.y;
            y_inv_exit = other.max.y - self.min.y;
        } else {
            y_inv_entry = other.max.y - self.min.y;
            y_inv_exit = other.min.y - self.max.y;
        }

        if linear_velocity.z > 0. {
            z_inv_entry = other.min.z - self.max.z;
            z_inv_exit = other.max.z - self.min.z;
        } else {
            z_inv_entry = other.max.z - self.min.z;
            z_inv_exit = other.min.z - self.max.z;
        }

        // find time of collision and time of leaving for each axis (if statement is to prevent divide by zero)
        let x_entry;
        let y_entry;
        let z_entry;
        let x_exit;
        let y_exit;
        let z_exit;

        // These new variables will give us our value between 0 and 1
        // of when each collision occurred on each axis.
        if linear_velocity.x == 0. {
            x_entry = f32::NEG_INFINITY;
            x_exit = f32::INFINITY;
            // do an overlap check
        } else {
            x_entry = x_inv_entry / linear_velocity.x;
            x_exit = x_inv_exit / linear_velocity.x;
        }

        if linear_velocity.y == 0. {
            y_entry = f32::NEG_INFINITY;
            y_exit = f32::INFINITY;
            // do an overlap check
        } else {
            y_entry = y_inv_entry / linear_velocity.y;
            y_exit = y_inv_exit / linear_velocity.y;
        }

        if linear_velocity.z == 0. {
            z_entry = f32::NEG_INFINITY;
            z_exit = f32::INFINITY;
            // do an overlap check
        } else {
            z_entry = z_inv_entry / linear_velocity.z;
            z_exit = z_inv_exit / linear_velocity.z;
        }

        // The next step is to find which axis collided first.
        // find the earliest/latest times of collision
        let entry_time = x_entry.max(y_entry).max(z_entry);
        let exit_time = x_exit.min(y_exit).min(z_exit);

        // if there was no collision
        if entry_time > exit_time || x_entry < 0. && y_entry < 0. || x_entry > 1. || y_entry > 1. {
            normal = Vec3::splat(0.);
            return (1., normal);
        } else {
            // if there was a collision
            // calculate normal of collided surface
            #[allow(clippy::float_cmp)]
            if y_entry == entry_time {
                if y_inv_entry < 0. {
                    normal.y = 1.;
                } else {
                    normal.y = -1.;
                }
            } else if x_entry == entry_time {
                if x_inv_entry < 0. {
                    normal.x = 1.0;
                } else {
                    normal.x = -1.;
                }
            } else if z_inv_entry < 0. {
                normal.z = 1.;
            } else {
                normal.z = -1.;
            }
        }

        // return the time of collision and the normal
        (entry_time, normal)
    }
}
