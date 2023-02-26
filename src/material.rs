use crate::{ray::Ray, hittable::HitRecord, vec::Color3};


pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color3, scattered: &mut Ray) -> bool;
}