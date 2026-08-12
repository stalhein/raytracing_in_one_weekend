
use crate::vec3::*;
use crate::ray::*;
use crate::interval::*;
use crate::geometry::hittable::*;
use std::cmp;
use std::fmt::Write;
use rand::RngExt;


pub struct Camera {
    samples_pp: i32,
    samples_scale: f64,
    max_depth: i32,
    image_width: i32,
    image_height: i32,
    _aspect_ratio: f64,
    centre: Point3,
    pixel100_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(width: i32, aspect: f64, samples: i32) -> Self {
        let height = cmp::max(1, (width as f64 / aspect) as i32);
        let c = Point3::new(0.0, 0.0, 0.0);

        // Viewport
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (width as f64/height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_du = viewport_u / width as f64;
        let pixel_dv = viewport_v / height as f64;

        let upper_left = c - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
        let upper_left_pixel = upper_left + (pixel_du + pixel_dv) * 0.5;

        Self {
            samples_pp: samples,
            samples_scale: 1.0 / samples as f64,
            max_depth: samples,
            image_width: width,
            image_height: height,
            _aspect_ratio: aspect,
            centre: c,
            pixel100_loc: upper_left_pixel,
            pixel_delta_u: pixel_du,
            pixel_delta_v: pixel_dv,
        }
    }


    pub fn render<W: Write>(&self, out: &mut W, world: &HittableList) {
        let _ = write!(out, "P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            println!("Lines remaining: {}", (self.image_height-j));
            for i in 0..self.image_width {
                let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);

                for _sample in 0..self.samples_pp {
                    let ray = self.get_ray(i, j);
                    pixel_colour = pixel_colour + self.ray_colour(&ray, self.max_depth, world);
                }

                pixel_colour = pixel_colour * self.samples_scale;

                self.write_colour(out, &pixel_colour);

                /*
                let pixel_centre = self.pixel100_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_centre - self.centre;
                let ray = Ray::new(&self.centre, &ray_direction);

                let pixel_colour = self.ray_colour(&ray, &world);
                self.write_colour(out, &pixel_colour);
                */
            }
        }
        println!("Done!");
    }

    fn write_colour<W: Write>(&self, out: &mut W, colour: &Colour) {
        let r = colour.r();
        let g = colour.g();
        let b = colour.b();

        let interval = Interval::new(0.0, 0.999);
        let rbyte = (256.0 * interval.clamp(r)) as u8;
        let gbyte = (256.0 * interval.clamp(g)) as u8;
        let bbyte = (256.0 * interval.clamp(b)) as u8;

        let _ = write!(out, "{} {} {}\n", rbyte, gbyte, bbyte);
    }

    fn ray_colour(&self, ray: &Ray, depth: i32, world: &HittableList) -> Colour {
        if depth <= 0 { return Colour::new(0.0, 0.0, 0.0); }

        if let Some(hit) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            let direction = Vec3::random_on_hemisphere(&hit.normal()) + Vec3::random_unit();
            return self.ray_colour(&Ray::new(&hit.position(), &direction), depth - 1, world) * 0.5;
        }

        let unit_direction = normalise(ray.direction());
        let a = (unit_direction.y() + 1.0) * 0.5;

        Colour::new(1.0, 1.0, 1.0)*(1.0-a) + Colour::new(0.5, 0.7, 0.9)*a
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel100_loc 
            + (self.pixel_delta_u * (i as f64 + offset.x()))
            + (self.pixel_delta_v * (j as f64 + offset.y()));

        let ray_origin = self.centre;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        let mut rng = rand::rng();
        let a: f64 = rng.random();
        let b: f64 = rng.random();
        Vec3::new(a - 0.5, b - 0.5, 0.0)
    }
}
