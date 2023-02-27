use std::rc::Rc;

use crate::{
    hittable::Hittable,
    material::{Material, NullMaterial},
    vec::Point3,
    vec_util::dot,
};

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: Point3::new(),
            radius: 0.0,
            material: Rc::new(NullMaterial {}),
        }
    }

    pub fn from(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn rc_from(center: Point3, radius: f64, material: Rc<dyn Material>) -> Rc<Self> {
        Rc::new(Self {
            center,
            radius,
            material,
        })
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let oc = ray.get_origin().clone() - self.center;
        let a = ray.get_direction().length_squared();
        let half_b = dot(&oc, ray.get_direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.material = self.material.clone();

        return true;
    }
}

pub struct SphereBuilder {
    sphere: Sphere,
}

impl SphereBuilder {
    pub fn new() -> Self {
        Self {
            sphere: Sphere::new(),
        }
    }

    pub fn set_position(&mut self, center: Point3) -> &mut Self {
        self.sphere.center = center;
        self
    }

    pub fn set_radius(&mut self, radius: f64) -> &mut Self {
        self.sphere.radius = radius;
        self
    }

    pub fn set_material(&mut self, material: Rc<dyn Material>) -> &mut Self {
        self.sphere.material = material;
        self
    }

    pub fn to_rc(&self) -> Rc<Sphere> {
        Rc::new(self.sphere.clone())
    }
}
