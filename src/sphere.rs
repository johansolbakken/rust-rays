use crate::{hittable::Hittable, vec::Point3, vec_util::dot};

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: Point3::new(),
            radius: 0.5,
        }
    }

    pub fn from(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn box_from(center: Point3, radius: f64) -> Box<Self> {
        Box::<Self>::from(Self { center, radius })
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

        return true;
    }
}
