use crate::api::vector::Vector3;
use crate::api::ray::Ray;
use crate::api::material::Material;

pub struct Hit {
    pub distance : f64,
    pub normal : Vector3,
    pub position : Vector3,
    pub object_id : usize 
}

pub trait Intersect {
    fn intersect(&self, ray : &Ray, object_id : &usize) -> Option<Hit>;
}
pub struct Sphere {
    pub radius : f64,
    pub position : Vector3,
    pub emission : Vector3,
    pub material : Material
}

impl Intersect for Sphere {
    fn intersect(&self, ray : &Ray, object_id : &usize) -> Option<Hit> {
        let p_o = &(self.position) - &(ray.org);
        let b = p_o.dot(&ray.dir);
        let d4 = b*b - p_o.length2() + self.radius * self.radius;

        if d4 < 0.0 {
            return None;
        };

        let sqrt_d4 = f64::sqrt(d4);
        let t1 = b - sqrt_d4;    
        let t2 = b + sqrt_d4;

        let eps = 1e-8;

        if t1 < eps && t2 < eps {
            return None;
        };

        let distance_ =  if t1 > eps {t1} else {t2};
        let position_ = &(ray.org) + &(distance_ * &(ray.dir)); 
        let normal_ = (&position_ - &(self.position)).normalize();

        Some(Hit{distance:distance_,normal:normal_,position:position_,object_id:object_id.clone()})
    }
}

pub enum Figure {
    Sphere(Sphere)
}