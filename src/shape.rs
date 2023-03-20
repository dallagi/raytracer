use crate::intersections::Intersections;
use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

pub mod sphere;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Sphere,
}

impl Shape {
    pub fn object_normal_at(self, object_point: Point) -> Vector {
        match self {
            Shape::Sphere => sphere::object_normal_at(object_point),
        }
    }

    pub fn object_intersect_at(self, object: Object, transformed_ray: Ray) -> Intersections {
        match object.shape {
            Shape::Sphere => sphere::object_intersect_at(object, transformed_ray),
        }
    }
}
