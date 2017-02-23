use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod linearalgebra;
mod tracing;

use linearalgebra::Vec3;
use tracing::Ray;

fn color(r: Ray) -> Vec3 {
    let unit = r.direction.unit_vector();
    let t: f32 = 0.5 * (unit.y + 1.0);
    (1.0 - t) *
    Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    } +
    t *
    Vec3 {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    }
}

fn write_example(mut f: File) {
    let nx = 200;
    let ny = 100;

    // Write the header.
    writeln!(&mut f, "P3").unwrap();
    writeln!(&mut f, "{} {}", nx, ny).unwrap();
    writeln!(&mut f, "255").unwrap();

    let lower_left_corner = Vec3 {
        x: -2.0,
        y: -1.0,
        z: -1.0,
    };
    let horizontal = Vec3 {
        x: 4.0,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: 2.0,
        z: 0.0,
    };
    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);
            let r = Ray {
                origin: origin,
                direction: lower_left_corner + u * horizontal + v * vertical,
            };
            let c = color(r);
            let ir = (255.99 * c.x) as i32;
            let ig = (255.99 * c.y) as i32;
            let ib = (255.99 * c.z) as i32;
            writeln!(&mut f, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }
}

fn main() {
    let path = Path::new("out.ppm");
    let display = path.display();

    let f = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}:", display, why.description()),
        Ok(f) => f,
    };

    write_example(f);
}
