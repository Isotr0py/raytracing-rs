use camera::CameraBuilder;
use hittable_list::HittableList;
use material::Material;
use vector::Vec3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vector;

fn main() {
    // World
    let mut world = HittableList::new();

    // Material
    let material_ground = Material::lambertian(Vec3::from_xyz(0.5, 0.5, 0.5));
    world.add(sphere::Sphere::new(
        Vec3::from_xyz(0., -1000., 0.),
        1000.,
        material_ground,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Vec3::from_xyz(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            if (center - Vec3::from_xyz(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Material;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    sphere_material = Material::lambertian(albedo);
                    world.add(sphere::Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rand::random::<f64>() * 0.5;
                    sphere_material = Material::metal(albedo, fuzz);
                    world.add(sphere::Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    sphere_material = Material::dielectric(1.5);
                    world.add(sphere::Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Material::dielectric(1.5);
    world.add(sphere::Sphere::new(
        Vec3::from_xyz(0., 1., 0.),
        1.0,
        material1,
    ));

    let material2 = Material::lambertian(Vec3::from_xyz(0.4, 0.2, 0.1));
    world.add(sphere::Sphere::new(
        Vec3::from_xyz(-4., 1., 0.),
        1.0,
        material2,
    ));

    let material3 = Material::metal(Vec3::from_xyz(0.7, 0.6, 0.5), 0.0);
    world.add(sphere::Sphere::new(
        Vec3::from_xyz(4., 1., 0.),
        1.0,
        material3,
    ));

    // Image
    let image_width: usize = 640;
    let image_height: usize = 360;
    let samples_per_pixel: usize = 10;
    let max_depth: usize = 50;

    // Camera
    let vfov: f64 = 20.0;
    let look_from = Vec3::from_xyz(13.0, 2.0, 3.0);
    let look_at = Vec3::from_xyz(0.0, 0.0, 0.0);
    let vup = Vec3::from_xyz(0., 1., 0.);

    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let mut cam = CameraBuilder::default()
        .image_size(image_width, image_height)
        .samples_per_pixel(samples_per_pixel)
        .max_depth(max_depth)
        .vfov(vfov)
        .look_from(look_from)
        .look_at(look_at)
        .vup(vup)
        .defocus_angle(defocus_angle)
        .focus_dist(focus_dist)
        .build()
        .unwrap();
    cam = cam.initialize();
    cam.render(&world);
}
