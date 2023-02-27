use std::rc::Rc;

use crate::{
    material::{Material, NullMaterial},
    ray::Ray,
    vec::{Point3, Vec3},
    vec_util::dot,
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(ray.get_direction(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = -outward_normal.clone();
        }
    }

    pub(crate) fn new() -> Self {
        Self {
            p: Point3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
            material: Rc::new(NullMaterial {}),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
