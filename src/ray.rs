use crate::vec::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Point3::new(),
            direction: Vec3::new(),
        }
    }
    pub fn from(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn get_origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn get_direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin.clone() + self.direction.clone() * t
    }
}
