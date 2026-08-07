mod vec3;
use vec3::*;
mod ray;
mod geometry;
use geometry::hittable::*;
mod interval;
mod camera;
use camera::*;

use std::io::Write as IoWrite;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;

    let mut buffer = String::new();

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera 
    let camera = Camera::new(image_width, aspect_ratio, 100);

    camera.render(&mut buffer, &world);

    let mut file = File::create("image.ppm")?;
    file.write_all(buffer.as_bytes())?;

    Ok(())
}
