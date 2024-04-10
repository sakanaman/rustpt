use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;
use crate::api::vector::Vector3;
use crate::api::ray::Ray;

pub trait SampleBSDF
{
   fn sample(&self, ray : &Ray, normal : &Vector3, rng : &mut ThreadRng)->(Vector3, Ray); // return (contribution, ray)
   fn pdf(&self, ray : &Ray, normal : &Vector3, next_ray : &Ray)->f64;
   fn bsdf(&self, ray : &Ray, normal : &Vector3, next_ray : &Ray)->Vector3;
}

pub struct Diffuse {
    pub albedo : Vector3
}

impl SampleBSDF for Diffuse {
    fn sample(&self, ray : &Ray, normal : &Vector3, rng : &mut ThreadRng)->(Vector3, Ray) {
        let u : f64 = rng.gen();
        let r2 : f64 = rng.gen();
        let r1 =  2.0 * PI * u;
        let r2s = f64::sqrt(r2);
        let tans = normal.onb(); 
        let nextvec =    tans.0 * f64::cos(r1) * r2s 
                                + tans.1 * f64::sin(r1) * r2s 
                                + normal * f64::sqrt(1.0 - r2) ;
        (self.albedo.clone(), Ray{org: ray.org.clone(), dir: nextvec})
    }
    fn pdf(&self, _ray : &Ray, normal : &Vector3, next_ray : &Ray)->f64 {
        let costheta = f64::abs(next_ray.dir.dot(normal));
        costheta/PI
    }
    fn bsdf(&self, _ray : &Ray, _normal : &Vector3, _next_ray : &Ray)->Vector3 {
       &self.albedo/PI
    }
}

pub enum Material {
   Diffuse(Diffuse) 
}