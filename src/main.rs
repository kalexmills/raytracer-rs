use std::fs::File;
use std::io::prelude::*;

#[macro_use] extern crate impl_ops;
use std::ops;

use vec3::Vector3;
use ray::Ray;
mod ray;
mod vec3;


fn main() {
    let nx:i64 = 200;
    let ny:i64 = 100;
    let mut a:String = format!("P3\n{} {}\n255\n", nx, ny);

    let lower_left_corner:Vector3 = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal:Vector3 = Vector3::new(4.0, 0.0, 0.0);
    let vertical:Vector3 = Vector3::new(0.0, 2.0, 0.0);
    let origin:Vector3 = Vector3::new(0.0, 0.0, 0.0);

    for y in (0..ny).rev() {
        for x in 0..nx {
            let u:f32 = x as f32 / nx as f32;
            let v:f32 = y as f32 / ny as f32;
            let r:Ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            let col:Vector3 = color(r);

            let ir:i64 = (255.99 * col.r()) as i64;
            let ig:i64 = (255.99 * col.g()) as i64;
            let ib:i64 = (255.99 * col.b()) as i64;
            a.push_str(&format!("{} {} {}\n", ir, ig, ib));
        }
    }
    writeToFile(&a);
}

fn hit_sphere(center: &Vector3, radius: f32, r: &Ray) {
    let oc: Vector3 = r.origin() - center;
    
}

fn color(r:Ray) -> Vector3 {
  let unit_direction:Vector3 = r.direction().unit_vector();
  let t:f32 = 0.5 * (unit_direction.y() + 1.0);
  return (1.0 - t) * Vector3::new(1.0,1.0,1.0) + t * Vector3::new(0.5, 0.7, 0.9);
}

fn writeToFile(s:&String) -> std::io::Result<()> {
    let mut file = File::create("TEXTMAN.ppm")?;
    file.write_all(s.as_bytes())?;
    Ok(())
}
