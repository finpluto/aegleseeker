use std::cell::OnceCell;

use glam::Vec3;

pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub color: Vec3,
    normal: OnceCell<Vec3>,
}

impl Triangle {
    pub const fn new(v0: Vec3, v1: Vec3, v2: Vec3, color: Vec3) -> Self {
        Self {
            v0,
            v1,
            v2,
            color,
            normal: OnceCell::new(),
        }
    }

    // This normal orientation is important,
    // a flipped normal will influence illumination model.
    pub fn get_normal(&self) -> Vec3 {
        *self
            .normal
            .get_or_init(|| ((self.v2 - self.v0).cross(self.v1 - self.v0)).normalize())
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
