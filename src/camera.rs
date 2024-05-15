use std::f64::INFINITY;
use std::io::{self, Write};

use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::{Lambertian, Material, Metal, Scatterable};
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
    max_depth: usize,         // Maximum depth of ray recursion
    pub vfov: f64,                // Vertical view angle (field of view)
    look_from: Vec3,          // Camera lookfrom point
    look_at: Vec3,            // Camera lookat point
    vup: Vec3,                // Camera view up vector
    u: Vec3,                  // Camera u vector
    v: Vec3,                  // Camera v vector
    w: Vec3,                  // Camera w vector
    defocus_angle: f64,       // Variation angle of rays through each pixel
    focus_dist: f64,          // Distance from camera lookfrom point to plane of perfect focus
    defocus_disk_u: Vec3,     // Defocus disk horizontal radius
    defocus_disk_v: Vec3,     // Defocus disk vertical radius
}

impl Camera {
    pub fn new(image_width: usize, aspect_ratio: f64, vfov: f64, samples_per_pixel: usize, max_depth: usize) -> Camera {
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        let pixel_sample_scale = 1.0 / samples_per_pixel as f64;

        let defocus_angle = 10.0;
        let focus_dist = 3.4;
        let look_from = Vec3::from_xyz(-2., 2., 1.);
        let look_at = Vec3::from_xyz(0., 0., -1.);
        let vup = Vec3::from_xyz(0., 1., 0.);
        let center = look_from;

        // Determine viewport dimensions.
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - focus_dist * w - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * (defocus_angle / 2.0 as f64).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
        Camera {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_sample_scale,
            samples_per_pixel,
            max_depth,
            vfov,
            look_from,
            look_at,
            vup,
            u,
            v,
            w,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
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
                    pixel_color += self.ray_color(&ray, self.max_depth, world);
                }
                write_color(self.pixel_sample_scale * pixel_color);
            }
        }
        eprintln!("\nDone.");
    }

    fn ray_color(&self, r: &Ray, depth:usize, world: &HittableList) -> Vec3 {
        if depth == 0 {return Vec3::zeros()}

        let mut rec = HitRecord::new();
        let color = match world.hit(r, Interval::new(1e-3, INFINITY), &mut rec) {
            true => {
                let mut scattered = Ray::new(Vec3::zeros(), Vec3::zeros());
                let mut attenuation = Vec3::ones();
                let mat = rec.mat.clone();
                if mat.scatter(&r, &mut rec, &mut attenuation, &mut scattered) {
                    attenuation * self.ray_color(&scattered, depth-1, world)
                } else {
                    Vec3::zeros()
                }
            },
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
        let pixel_sample = self.pixel00_loc
            + ((i + offset.x()) * self.pixel_delta_u)
            + ((j + offset.y()) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0. {self.center} else {self.defocus_disk_sample()};
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::from_xyz(rand::random::<f64>() - 0.5, rand::random::<f64>() + 0.5, 0.)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}
