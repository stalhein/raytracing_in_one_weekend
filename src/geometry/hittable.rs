use crate::vec3::*;
use crate::ray::*;
use crate::interval::*;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    position: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, n: Vec3, a: f64, front: bool) -> Self {
        Self {
            position: p,
            normal: n,
            t: a,
            front_face: front,
        }
    }

    pub fn position(&self) -> Point3 {
        self.position
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t 
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

fn is_front_face(ray: Ray, outward_normal: Vec3) -> bool {
    dot(ray.direction(), &outward_normal) < 0.0
}

fn get_normal(outward_normal: Vec3, front_face: bool) -> Vec3 {
    outward_normal * (front_face as i32 * 2 - 1) as f64
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn add(&mut self, item: Box<dyn Hittable>) {
        self.objects.push(item);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        let mut closest = t.max();
        let mut best_hit = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, Interval::new(t.min(), closest)) {
                best_hit = Some(hit);
                closest = hit.t();
            }
        }

        best_hit
    }
}

pub struct Sphere {
    centre: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(c: Point3, r: f64) -> Self {
        Self { centre: c, radius: r }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        let oc = self.centre - *ray.origin();
        let a = ray.direction().length_squared();
        let h = dot(ray.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();

        let mut root = (h - discriminant_sqrt) / a;
        if !t.surrounds(root) {
            root = (h + discriminant_sqrt) / a;
            if !t.surrounds(root) {
                return None;
            }
        }

        let position = ray.at(root);
        let outward_normal = (position - self.centre) / self.radius;

        let front_face = is_front_face(*ray, outward_normal);

        Some(HitRecord::new(
            position,
            get_normal(outward_normal, front_face),
            root,
            front_face,
        ))
    }
}
