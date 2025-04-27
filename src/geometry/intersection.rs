use glam::{Vec3, mat3};

use crate::geometry::primitives::{Ray, Triangle};

pub trait Intersect<T: 'static> {
    type Intersection<'t>: Intersection;
    fn intersect<'t>(&self, l: &'t T) -> Option<Self::Intersection<'t>>;
}

impl Intersect<Triangle> for Ray {
    type Intersection<'t> = TriangleIntersection<'t>;

    fn intersect<'t>(&self, l: &'t Triangle) -> Option<Self::Intersection<'t>> {
        let e1 = l.v1 - l.v0;
        let e2 = l.v2 - l.v0;
        let b = self.origin - l.v0;
        let a = mat3(-self.direction, e1, e2);

        if a.determinant().abs() <= 1e-4 {
            return None;
        }

        let tuv = a.inverse() * b;

        let position = self.origin + tuv.x * self.direction;

        Some(TriangleIntersection {
            position,
            distance: tuv.x,
            tuv,
            hit_obj: l,
        })
        .filter(|intersection| !intersection.distance.is_nan() && intersection.is_inside())
    }
}

pub trait Intersection {
    fn is_inside(&self) -> bool;
    fn get_color(&self) -> Vec3;
    fn get_normal(&self) -> Vec3;
    fn get_hit_point(&self) -> Vec3;
    fn get_distance(&self) -> f32;
}

impl Intersection for TriangleIntersection<'_> {
    fn is_inside(&self) -> bool {
        let tuv = self.tuv;
        // Pretty hack 1e-4 here too.
        // To avoid some weird situation that happens when two triangles share a same edge.
        tuv.x >= 1e-4 && tuv.y >= 0.0 && tuv.z >= 0.0 && tuv.y + tuv.z - 1f32 <= 1e-4
    }

    fn get_color(&self) -> Vec3 {
        self.hit_obj.color
    }

    fn get_normal(&self) -> Vec3 {
        self.hit_obj.get_normal()
    }

    fn get_hit_point(&self) -> Vec3 {
        self.position
    }

    fn get_distance(&self) -> f32 {
        self.distance
    }
}

pub struct TriangleIntersection<'t> {
    pub position: Vec3,
    pub tuv: Vec3,
    pub distance: f32,
    pub hit_obj: &'t Triangle,
}

pub trait Tracible<R> {
    type Intersection: Intersection;

    fn closest_intersection(self, ray: &R) -> Option<Self::Intersection>;
}

impl<'o, Scene> Tracible<Ray> for &'o Scene
where
    &'o Scene: IntoIterator<Item = &'o Triangle>,
{
    type Intersection = TriangleIntersection<'o>;

    fn closest_intersection(self, ray: &Ray) -> Option<Self::Intersection> {
        self.into_iter()
            .filter_map(|obj| ray.intersect(obj))
            .min_by(|l, r| l.distance.partial_cmp(&r.distance).unwrap())
    }
}
