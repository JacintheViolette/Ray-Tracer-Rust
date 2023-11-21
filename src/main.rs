mod vec3;
mod color;
mod ray;

use std::io::{self, Write};

use crate::{color::{write_color, Color, ray_color}, vec3::Vec3, ray::Ray};

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
 
    let width: i32 = 400;
    let mut height: i32 = (width as f32 / aspect_ratio) as i32;

    if height < 1 {
        height = 1
    }

    
    // Caméra
    
    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * (width as f32 / height as f32);
    let camera_center: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    
    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / width as f32;
    let pixel_delta_v = viewport_v / height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


    println!("P3\n{} {}\n255", width, height);

    for j in 0..height {
        for i in 0..width {
            /*
            let r = i as f32 / (width as f32 - 1 as f32);
            let g = j as f32 / (height as f32 - 1 as f32);
            let b = 0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b as f32) as i32;
            */

            let pixel_center = pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let r = Ray::new(camera_center, ray_direction);

            //writeln!(io::stderr(), "ray : {:?}", r).expect("Bah frérot tu troll aussi");

            let color = ray_color(&r);

            write_color(color)
        }
        writeln!(io::stderr(), "Generating image: {}%", ((j as f32 * width as f32) * 100 as f32) / (height as f32 * width as f32)).expect("Bah frérot tu troll aussi");
    }
    writeln!(io::stderr(), "Done").expect("Erreur lors de l'écriture sur stderr");
}
