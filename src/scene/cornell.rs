use crate::geometry::primitives::Triangle;

use super::colors::*;
use glam::Vec3;

// Length of Cornell Box side.
const L: f32 = 555.0;

#[allow(clippy::declare_interior_mutable_const)]
pub const ROOM: [Triangle; 10] = {
    const A: Vec3 = Vec3::new(L, 0.0, 0.0);
    const B: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    const C: Vec3 = Vec3::new(L, 0.0, L);
    const D: Vec3 = Vec3::new(0.0, 0.0, L);
    const E: Vec3 = Vec3::new(L, L, 0.0);
    const F: Vec3 = Vec3::new(0.0, L, 0.0);
    const G: Vec3 = Vec3::new(L, L, L);
    const H: Vec3 = Vec3::new(0.0, L, L);

    [
        Triangle::new(C, B, A, GREEN),
        Triangle::new(C, D, B, GREEN),
        Triangle::new(A, E, C, PURPLE),
        Triangle::new(C, E, G, PURPLE),
        Triangle::new(F, B, D, YELLOW),
        Triangle::new(H, F, D, YELLOW),
        Triangle::new(E, F, G, CYAN),
        Triangle::new(F, H, G, CYAN),
        Triangle::new(G, D, C, WHITE),
        Triangle::new(G, H, D, WHITE),
    ]
};

#[allow(clippy::declare_interior_mutable_const)]
pub const SHORT_BLOCK: [Triangle; 10] = {
    const A: Vec3 = Vec3::new(290.0, 0.0, 114.0);
    const B: Vec3 = Vec3::new(130.0, 0.0, 65.0);
    const C: Vec3 = Vec3::new(240.0, 0.0, 272.0);
    const D: Vec3 = Vec3::new(82.0, 0.0, 225.0);
    const E: Vec3 = Vec3::new(290.0, 165.0, 114.0);
    const F: Vec3 = Vec3::new(130.0, 165.0, 65.0);
    const G: Vec3 = Vec3::new(240.0, 165.0, 272.0);
    const H: Vec3 = Vec3::new(82.0, 165.0, 225.0);
    [
        Triangle::new(E, B, A, RED),
        Triangle::new(E, F, B, RED),
        Triangle::new(F, D, B, RED),
        Triangle::new(F, H, D, RED),
        Triangle::new(H, C, D, RED),
        Triangle::new(H, G, C, RED),
        Triangle::new(G, E, C, RED),
        Triangle::new(E, A, C, RED),
        Triangle::new(G, F, E, RED),
        Triangle::new(G, H, F, RED),
    ]
};

#[allow(clippy::declare_interior_mutable_const)]
pub const TALL_BLOCK: [Triangle; 10] = {
    const A: Vec3 = Vec3::new(423.0, 0.0, 247.0);
    const B: Vec3 = Vec3::new(265.0, 0.0, 296.0);
    const C: Vec3 = Vec3::new(472.0, 0.0, 406.0);
    const D: Vec3 = Vec3::new(314.0, 0.0, 456.0);
    const E: Vec3 = Vec3::new(423.0, 330.0, 247.0);
    const F: Vec3 = Vec3::new(265.0, 330.0, 296.0);
    const G: Vec3 = Vec3::new(472.0, 330.0, 406.0);
    const H: Vec3 = Vec3::new(314.0, 330.0, 456.0);
    [
        Triangle::new(E, B, A, BLUE),
        Triangle::new(E, F, B, BLUE),
        Triangle::new(F, D, B, BLUE),
        Triangle::new(F, H, D, BLUE),
        Triangle::new(H, C, D, BLUE),
        Triangle::new(H, G, C, BLUE),
        Triangle::new(G, E, C, BLUE),
        Triangle::new(E, A, C, BLUE),
        Triangle::new(G, F, E, BLUE),
        Triangle::new(G, H, F, BLUE),
    ]
};

pub fn scale_triangle(mut t: Triangle) -> Triangle {
    t.v0 *= 2f32 / L;
    t.v1 *= 2f32 / L;
    t.v2 *= 2f32 / L;

    t.v0 -= Vec3::splat(1f32);
    t.v1 -= Vec3::splat(1f32);
    t.v2 -= Vec3::splat(1f32);

    let xy_flipping = Vec3::new(-1f32, -1f32, 1f32);
    t.v0 *= xy_flipping;
    t.v1 *= xy_flipping;
    t.v2 *= xy_flipping;
    t
}
