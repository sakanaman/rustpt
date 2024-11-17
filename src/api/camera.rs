use crate::api::vector::Vector3;
use std::f64::consts::PI;
use super::ray::Ray;

pub trait Camera {
    fn generate_first_ray(&self, i:usize, j:usize)->Ray;
}

pub struct PinholeCamera {
    pub cam_pos : Vector3,  
    pub cam_forward : Vector3, // must be norm = 1
    pub cam_right: Vector3, // must be norm = 1
    pub cam_up : Vector3,// must be norm = 1
    pub screen_width : f64,
    pub screen_height : f64,
    pub aov : f64, // [rad]
    pub pixel_size : f64
}

pub fn new_edupt(width: usize, height: usize) -> PinholeCamera {
    let cam_pos =  Vector3{x:50.0, y:50.0, z:220.0};
    let cam_up =  Vector3{x:0.0, y:1.0, z:0.0};
    let cam_forward = Vector3{x:0.0, y:-0.04, z:-1.0}.normalize();
    let cam_right = cam_up.cross(&cam_forward).normalize();
    let screen_width = 30.0 * (width as f64) / (height as f64);
    let screen_height = 30.0;
    let aov= PI * 0.25;
    let pixel_size = screen_height / (height as f64);

    PinholeCamera{
        cam_pos,
        cam_forward,
        cam_right,
        cam_up,
        screen_width,
        screen_height,
        aov,
        pixel_size
    }
}

impl Camera for PinholeCamera {
    fn generate_first_ray(&self, i:usize, j:usize)->Ray {
        let dist_censor_to_pos = self.screen_width*0.5 / f64::tan(self.aov*0.5);
        let censor_pos = &self.cam_pos + &(dist_censor_to_pos * &self.cam_forward);
        let left_up_pos = censor_pos 
                                 + 0.5 * (&(&self.cam_up * self.screen_height) - &(&self.cam_right * self.screen_width));
        let start_pos = left_up_pos + j as f64 * self.pixel_size * &self.cam_right
                                             - i as f64 * self.pixel_size * &self.cam_up;
        Ray { org: self.cam_pos.clone(), dir: (&start_pos - &self.cam_pos).normalize() }
    }
}

