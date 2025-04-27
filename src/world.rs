use dashmap::DashMap;
use glam::Vec3;

use crate::geometry::intersection::{Intersection, Tracible};
use crate::geometry::primitives::Triangle;
use crate::light::Light;
use crate::scene::camera::{Camera, PixelCoordinate};
use crate::scene::cornell::{self, ROOM, SHORT_BLOCK, TALL_BLOCK};

pub struct World {
    pub objs: Vec<Triangle>,
    pub camera: Camera,
    pub pixel_map: DashMap<PixelCoordinate, Vec3>,
    pub light: Light,
}

impl World {
    pub fn new(canvas_height: u32, canvas_width: u32) -> Self {
        let objs = vec![ROOM, TALL_BLOCK, SHORT_BLOCK]
            .into_flattened()
            .into_iter()
            .map(cornell::scale_triangle)
            .collect();
        World {
            camera: Camera::new(
                canvas_height,
                canvas_width,
                Vec3::new(
                    //canvas_width as f32 / 2f32,
                    //canvas_height as f32 / 2f32,
                    0f32, 0f32, -3f32,
                ),
            ),
            objs,
            pixel_map: DashMap::new(),
            light: Light::default(),
        }
    }

    pub fn tracing(&self) {
        self.pixel_map.clear();

        for (pixel_pos, ray) in &self.camera {
            let color = match self.objs.closest_intersection(&ray) {
                Some(intersection) => {
                    //intersection.get_color()
                    intersection.get_color()
                        * (self.light.direct_light(&intersection, &self.objs)
                            + self.light.indirect_light())
                }
                None => Vec3::ZERO,
            };
            self.pixel_map.insert(pixel_pos, color);
        }
    }

    pub fn dump_pixels(&self, buf: &mut [u8]) {
        let scanline = self.camera.scanline_iter();
        for (x, y) in scanline {
            let idx_offset = y * self.camera.height + x;
            let byte_offset = idx_offset as usize * 4;
            let color = self
                .pixel_map
                .get(&(x, y).into())
                .map_or_else(|| Vec3::ZERO, |color_ref| *color_ref);

            let rgb_8bits_color = (color * 255.0).clamp(Vec3::ZERO, Vec3::splat(255f32));

            match buf.get_mut(byte_offset..byte_offset + 4) {
                Some(write_ref) => {
                    let mut color = rgb_8bits_color.as_u8vec3().to_array();
                    color.reverse();
                    // BGR
                    write_ref[0..3].clone_from_slice(&color);
                    // A
                    write_ref[3] = 255;
                }
                None => continue,
            }
        }
    }
}
