use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec::{Color3, Vec3},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct NullMaterial {}

impl Material for NullMaterial {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color3,
        scattered: &mut Ray,
    ) -> bool {
        return false;
    }
}

pub struct Lambertian {
    albedo: Color3,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color3,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate ray direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::from(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}
