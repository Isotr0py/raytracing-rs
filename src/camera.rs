use std::f64::INFINITY;
use std::io::{self, Write};

use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::Vec3;

use crate::color::write_color;

pub struct Camera {
    image_width: usize,       // Rendered image width
    image_height: usize,      // Rendered image height
    center: Vec3,             // Camera center
    pixel00_loc: Vec3,        // Location of pixel 0, 0
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
    pixel_sample_scale: f64,  // Scale factor for pixel samples
    samples_per_pixel: usize, // Count of random samples for each pixel
}

impl Camera {
    pub fn new(image_width: usize, aspect_ratio: f64, samples_per_pixel: usize) -> Camera {
        let center = Vec3::zeros();
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        let pixel_sample_scale = 1.0 / samples_per_pixel as f64;

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::from_xyz(viewport_width, 0., 0.);
        let viewport_v = Vec3::from_xyz(0., -viewport_height, 0.);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vec3::from_xyz(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        Camera {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_sample_scale,
            samples_per_pixel,
        }
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}", self.image_height - j);
            io::stdout().flush().unwrap();
            for i in 0..self.image_width {
                let mut pixel_color = Vec3::zeros();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i as f64, j as f64);
                    pixel_color += self.ray_color(&ray, world);
                }
                write_color(self.pixel_sample_scale * pixel_color);
            }
        }
        eprintln!("\nDone.");
    }

    fn ray_color(&self, r: &Ray, world: &HittableList) -> Vec3 {
        let mut rec = HitRecord::new();
        let color = match world.hit(r, Interval::new(0., INFINITY), &mut rec) {
            true => 0.5 * (rec.normal + Vec3::ones()),
            false => {
                let unit_direction = r.direction().unit_vector();
                let t = 0.5 * (unit_direction.y() + 1.0);
                Vec3::ones() * (1.0 - t) + Vec3::from_rgb(0.5, 0.7, 1.0) * t
            }
        };
        color
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = self.sample_square();
        let pixel_per_sample = self.pixel00_loc
            + ((i + offset.x()) * self.pixel_delta_u)
            + ((j + offset.y()) * self.pixel_delta_v);
        Ray::new(self.center, pixel_per_sample - self.center)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::from_xyz(rand::random::<f64>() - 0.5, rand::random::<f64>() + 0.5, 0.)
    }
}
