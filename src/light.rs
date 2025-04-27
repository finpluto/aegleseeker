use std::f32::consts::PI;

use glam::Vec3;

use crate::geometry::{
    intersection::{Intersection, Tracible},
    primitives::Ray,
};

pub struct Light {
    source: Vec3,
    direct_power: Vec3,
    indirect_power: Vec3,
    offset: Vec3,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            source: Vec3::new(0f32, -0.5, -0.7),
            direct_power: Vec3::splat(14f32),
            indirect_power: Vec3::splat(0.5),
            offset: Vec3::ZERO,
        }
    }
}

impl Light {
    pub fn direct_light<I, O>(&self, intersection: &I, tracible: O) -> Vec3
    where
        I: Intersection,
        O: Tracible<Ray>,
    {
        let r = self.get_source() - intersection.get_hit_point();
        let n = intersection.get_normal();
        let d = (r.normalize().dot(n)).max(0f32) / (4f32 * PI * r.dot(r)) * self.direct_power;

        let light_ray = self.light_ray(intersection);
        match tracible.closest_intersection(&light_ray) {
            Some(i) => {
                let intersection_d = i.get_distance();

                if intersection_d > 0f32 && r.length() > intersection_d {
                    Vec3::splat(0f32)
                } else {
                    d
                }
            }
            None => d,
        }
    }

    pub fn indirect_light(&self) -> Vec3 {
        self.indirect_power
    }

    fn light_ray<I: Intersection>(&self, intersection: &I) -> Ray {
        // IMPORTANT!: https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/ligth-and-shadows.html
        // a hack to eliminate shadow-acne
        // shift origin a little bit toward normal direction.
        // This shift amount (1e-4) comes from DD2258 visgra raytracing lab.
        let origin = intersection.get_hit_point() + 1e-4 * intersection.get_normal();
        let direction = (self.get_source() - origin).normalize();
        Ray { origin, direction }
    }

    pub fn update_offset(&mut self, x: f32, y: f32, z: f32) {
        self.offset = Vec3::new(x, y, z);
    }

    fn get_source(&self) -> Vec3 {
        self.source + self.offset
    }
}
