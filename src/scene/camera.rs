use glam::{Mat3, Vec3};

use crate::geometry::primitives::Ray;

pub struct Camera {
    pub width: u32,
    pub height: u32,
    world_position: Vec3,
    focal: f32,
    //rotation: Mat3,
    yaw: f32,
    z_translate: f32,
}

#[derive(PartialEq, Eq, Hash)]
pub struct PixelCoordinate {
    width_idx: u32,
    height_idx: u32,
}

impl From<(u32, u32)> for PixelCoordinate {
    fn from(value: (u32, u32)) -> Self {
        Self {
            width_idx: value.0,
            height_idx: value.1,
        }
    }
}

pub struct RayGenerator<'a> {
    camera: &'a Camera,
    scanline: ScanlineIter,
    rotate_mat_precalculated: Mat3,
}

impl Iterator for RayGenerator<'_> {
    type Item = (PixelCoordinate, Ray);

    fn next(&mut self) -> Option<Self::Item> {
        self.scanline.next().map(|(x, y)| {
            let camera_pos = self.camera.world_position;
            let focal = self.camera.focal;
            let width = self.camera.width as f32;
            let height = self.camera.height as f32;
            let rot = self.rotate_mat_precalculated;

            let new_x = rot.col(0);
            let new_y = rot.col(1);
            let new_z = rot.col(2);

            let direction = (x as f32 - (width / 2f32)) * new_x
                + (y as f32 - (height / 2f32)) * new_y
                + focal * new_z;

            let ray = Ray {
                origin: camera_pos + Vec3::new(0f32, 0f32, self.camera.z_translate),
                //direction: Vec3::new(x as f32 - (width / 2.0), y as f32 - (height / 2.0), focal),
                direction,
            };
            (
                PixelCoordinate {
                    width_idx: x,
                    height_idx: y,
                },
                ray,
            )
        })
    }
}

pub struct ScanlineIter {
    cur_width_idx: u32,
    cur_height_idx: u32,
    width: u32,
    height: u32,
}

impl ScanlineIter {
    pub fn new(height: u32, width: u32) -> Self {
        Self {
            cur_width_idx: 0,
            cur_height_idx: 0,
            width,
            height,
        }
    }
}

impl Iterator for ScanlineIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.cur_width_idx += 1;
        if self.cur_width_idx >= self.width {
            self.cur_width_idx = 0;
            self.cur_height_idx += 1;
        }
        if self.cur_height_idx >= self.height {
            return None;
        }
        Some((self.cur_width_idx, self.cur_height_idx))
    }
}

impl Camera {
    pub fn new(canvas_height: u32, canvas_width: u32, pos: Vec3) -> Self {
        Self {
            width: canvas_width,
            height: canvas_height,
            world_position: pos,
            focal: canvas_width as f32,
            yaw: 0f32,
            z_translate: 0f32,
        }
    }

    pub fn casting_rays(&self) -> RayGenerator {
        RayGenerator {
            camera: self,
            scanline: self.scanline_iter(),
            rotate_mat_precalculated: Mat3::from_rotation_y(self.yaw),
        }
    }

    pub fn scanline_iter(&self) -> ScanlineIter {
        ScanlineIter::new(self.height, self.width)
    }

    pub fn get_canvase_size(&self) -> u32 {
        self.height * self.width
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
    }

    pub fn set_z_translate(&mut self, offset: f32) {
        self.z_translate = offset;
    }
}

impl<'a> IntoIterator for &'a Camera {
    type Item = (PixelCoordinate, Ray);

    type IntoIter = RayGenerator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.casting_rays()
    }
}
