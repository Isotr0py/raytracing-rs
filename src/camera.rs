use std::f64::INFINITY;
use std::io::{self, Write};

use derive_builder::Builder;

use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::vector::Vec3;

use crate::color::write_color;

#[derive(Builder, Clone, Copy)]
#[builder(setter(skip))]
pub struct Camera {
    #[builder(setter)]
    image_width: usize, // Rendered image width
    #[builder(setter)]
    image_height: usize, // Rendered image height
    center: Vec3,            // Camera center
    pixel00_loc: Vec3,       // Location of pixel 0, 0
    pixel_delta_u: Vec3,     // Offset to pixel to the right
    pixel_delta_v: Vec3,     // Offset to pixel below
    pixel_sample_scale: f64, // Scale factor for pixel samples
    #[builder(setter)]
    samples_per_pixel: usize, // Count of random samples for each pixel
    #[builder(setter)]
    max_depth: usize, // Maximum depth of ray recursion
    #[builder(setter)]
    vfov: f64, // Vertical view angle (field of view)
    #[builder(setter)]
    look_from: Vec3, // Camera lookfrom point
    #[builder(setter)]
    look_at: Vec3, // Camera lookat point
    #[builder(setter)]
    vup: Vec3, // Camera view up vector
    u: Vec3,                 // Camera u vector
    v: Vec3,                 // Camera v vector
    w: Vec3,                 // Camera w vector
    #[builder(setter)]
    defocus_angle: f64, // Variation angle of rays through each pixel
    #[builder(setter)]
    focus_dist: f64, // Distance from camera lookfrom point to plane of perfect focus
    defocus_disk_u: Vec3,    // Defocus disk horizontal radius
    defocus_disk_v: Vec3,    // Defocus disk vertical radius
}

impl CameraBuilder {
    pub fn image_size(&mut self, width: usize, height: usize) -> &mut CameraBuilder {
        self.image_width(width).image_height(height)
    }
}

impl Camera {
    pub fn initialize(&mut self) -> Camera {
        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        // let defocus_angle = 10.0;
        // let focus_dist = 3.4;
        // let look_from = Vec3::from_xyz(-2., 2., 1.);
        // let look_at = Vec3::from_xyz(0., 0., -1.);
        // let vup = Vec3::from_xyz(0., 1., 0.);

        self.center = self.look_from.clone();

        // Determine viewport dimensions.
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (self.look_from - self.look_at).unit_vector();
        let u = self.vup.cross(w).unit_vector();
        let v = w.cross(u);
        self.u = u;
        self.v = v;
        self.w = w;

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / self.image_height as f64;
        self.pixel_delta_u = pixel_delta_u;
        self.pixel_delta_v = pixel_delta_v;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - self.focus_dist * w - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        self.pixel00_loc = pixel00_loc;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0 as f64).to_radians().tan();
        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;
        *self
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

    fn ray_color(&self, r: &Ray, depth: usize, world: &HittableList) -> Vec3 {
        if depth == 0 {
            return Vec3::zeros();
        }

        let mut rec = HitRecord::new();
        let color = match world.hit(r, Interval::new(1e-3, INFINITY), &mut rec) {
            true => {
                let mut scattered = Ray::new(Vec3::zeros(), Vec3::zeros());
                let mut attenuation = Vec3::ones();
                let mat = rec.mat.clone();
                if mat.scatter(&r, &mut rec, &mut attenuation, &mut scattered) {
                    attenuation * self.ray_color(&scattered, depth - 1, world)
                } else {
                    Vec3::zeros()
                }
            }
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
        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
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
