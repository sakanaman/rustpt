use crate::api::vector::Vector3;

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

impl Camera for PinholeCamera {
    fn generate_first_ray(&self, i:usize, j:usize)->Ray {
        let dist_censor_to_pos = self.screen_width*0.5 / f64::tan(self.aov*0.5);
        let censor_pos = &(self.cam_pos) + &(dist_censor_to_pos * &self.cam_forward);
        let left_up_pos = censor_pos 
                                 + 0.5 * (&(&(self.cam_up) * self.screen_height) - &(&(self.cam_right) * self.screen_width));
        let start_pos = left_up_pos + j as f64 * self.pixel_size * &(self.cam_right)
                                             - i as f64 * self.pixel_size * &(self.cam_up);
        Ray { org: self.cam_pos.clone(), dir: (&start_pos - &(self.cam_pos)).normalize() }
    }
}

