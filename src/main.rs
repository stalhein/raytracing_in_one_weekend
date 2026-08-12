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
use std::io;

fn get_input_int(default: i32) -> i32 {
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("Invalid input - set to {default}");
    match i.trim().parse() {
        Ok(num) => { return num; },
        Err(_) => {
            println!("Invalid input - set to {default}");
            return default;
        }
    };
}

fn main() -> std::io::Result<()> {
    // Width
    print!("Enter desired image width (higher takes longer - default is 800): ");
    let _ = io::stdout().flush();
    let w = get_input_int(800);

    // Height
    print!("Enter desired image height (again higher takes longer - default is 600): ");
    let _ = io::stdout().flush();
    let h = get_input_int(600);

    // Rpp
    print!("Enter number of rays per pixel, larger numbers reduce noise but take longer (default is 8): ");
    let _ = io::stdout().flush();
    let r = get_input_int(8);

    let aspect_ratio: f64 = w as f64 / h as f64;

    let mut buffer = String::new();

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera 
    let camera = Camera::new(w as i32, aspect_ratio, r as i32);

    camera.render(&mut buffer, &world);

    let mut file = File::create("image.ppm")?;
    file.write_all(buffer.as_bytes())?;

    Ok(())
}
