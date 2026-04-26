use parry2d::bounding_volume::{Aabb, BoundingVolume};
use parry2d::na::Point2;

use crate::custom::Point;

pub struct Collision {
    pub obj1: Aabb,
    pub obj2: Aabb,
}

impl Collision {
    pub fn intersect(&self) -> bool {
        self.obj1.intersects(&self.obj2)
    }
}

pub fn aabb_from_pos_size(pos: Point, size: Point) -> Aabb {
    Aabb {
        mins: Point2::new(pos.x, pos.y),
        maxs: Point2::new(pos.x + size.x, pos.y + size.y),
    }
}

pub fn padded_aabb_from_pos_size(pos: Point, size: Point, padding: f32) -> Aabb {
    Aabb {
        mins: Point2::new(pos.x + padding, pos.y + padding),
        maxs: Point2::new(pos.x + size.x - padding, pos.y + size.y - padding),
    }
}
