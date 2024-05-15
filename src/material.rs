use rand::random;

use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;

pub trait Scatterable {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo,
        }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random().unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo.clone();
        true
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        let fuzz = fuzz.min(1.).max(0.);
        Self {
            albedo,
            fuzz
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut reflected = r_in.direction().reflect(rec.normal);
        reflected = reflected.unit_vector() + self.fuzz * Vec3::random_in_unit_sphere();
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self {
            ref_idx,
        }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Scatterable for Dielectric{
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let ri = if rec.front_face { 1. / self.ref_idx } else { self.ref_idx };
        
        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = match cannot_refract || (&Dielectric::reflectance(cos_theta, ri) > &rand::random::<f64>()){
            true => unit_direction.reflect(rec.normal),
            false => unit_direction.refract(rec.normal, ri),
        };

        *attenuation = Vec3::ones();
        *scattered = Ray::new(rec.p, direction);
        true
    }
}

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn lambertian(albedo: Vec3) -> Self {
        Material::Lambertian(Lambertian::new(albedo))
    }

    pub fn metal(albedo: Vec3, fuzz: f64) -> Self {
        Material::Metal(Metal::new(albedo, fuzz))
    }

    pub fn dielectric(ref_idx: f64) -> Self {
        Material::Dielectric(Dielectric::new(ref_idx))
    }
}

impl Scatterable for Material {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match &self {
            Material::Lambertian(lambertian) => lambertian.scatter(r_in, rec, attenuation, scattered),
            Material::Metal(metal) => metal.scatter(r_in, rec, attenuation, scattered),
            Material::Dielectric(dielectric) => dielectric.scatter(r_in, rec, attenuation, scattered),
        }
    }
}