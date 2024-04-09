mod api;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::thread::spawn;

use api::raytrace::ray_trace;
use api::{figure::Figure, material::Material, search::LinearSearch, vector::Vector3};
use api::material::Diffuse;

use crate::api::camera::Camera;
use crate::api::camera::PinholeCamera;
use crate::api::figure::Sphere;
use image;

fn main() {
   let scene = 
    vec![
            Figure::Sphere(
                    Sphere{radius:1.0, 
                           position:Vector3{x:1.0,y:1.0,z:1.0},
                           emission:Vector3{x:1.0,y:1.0,z:1.0}, 
                           material:Material::Diffuse(Diffuse{albedo: Vector3{x:0.5,y:0.5,z:0.5} })}
            ),
        ];
    let searcher = Arc::new(LinearSearch{figures:scene});
    let width = 225;
    let height = 225;
    // let aspect_ratio = width as f64 / height as f64;
    let spp = 100;
    const THREAD_NUM : i32 = 8;

    let data = Arc::new(Mutex::new(vec![0.0_f64;height*width*3]));

    let mut thrd:Vec<JoinHandle<()>> = Vec::new();

    let pcam = Arc::new(PinholeCamera{cam_pos:Vector3{ x: 0.0, y: 0.0, z: 0.0 },
                                                                cam_forward:Vector3{ x: 0.0, y: 0.0, z: 0.0 },
                                                                cam_right:Vector3{ x: 0.0, y: 0.0, z: 0.0 },
                                                                cam_up:Vector3{ x: 0.0, y: 0.0, z: 0.0 },
                                                                screen_width:0.0,
                                                                screen_height:0.0,
                                                                aov:0.0,
                                                                pixel_size:0.0
                                                            });

    for thread_id in 0..THREAD_NUM {
        let data_local = Arc::clone(&data);
        let pcam_local = Arc::clone(&pcam);
        let searcher_local = Arc::clone(&searcher);
        let task =  
        move || {
            let mut rng = rand::thread_rng();
            for sample_id in 1..spp+1 {
                for i in ((thread_id as usize)..height).step_by(THREAD_NUM as usize) {
                    for j in 0..width {
                        let index =  i*width*3 + j*3;
                        let result = ray_trace(pcam_local.generate_first_ray(i, j), &searcher_local, &mut rng);
                        let mut locked_data = data_local.lock().unwrap();
                        locked_data[index] =     (1.0/sample_id as f64)*((sample_id as f64 - 1.0)*locked_data[index    ] + result.x);
                        locked_data[index + 1] = (1.0/sample_id as f64)*((sample_id as f64 - 1.0)*locked_data[index + 1] + result.y);
                        locked_data[index + 2] = (1.0/sample_id as f64)*((sample_id as f64 - 1.0)*locked_data[index + 2] + result.z);
                    }
                }
            }
        };
        thrd.push(spawn(task));
    }
    thrd.into_iter().for_each(|th| {
        let _ = th.join();
    });

    let mut data_image = image::RgbImage::new(width.clone() as u32, height.clone() as u32);

    let locked_data = data.lock().unwrap();
    for (x, y, pixel) in data_image.enumerate_pixels_mut() {
        let index =  y*width as u32*3 + x*3;
        *pixel = image::Rgb([(locked_data[index as usize    ] * 255.0) as u8, 
                             (locked_data[index as usize + 1] * 255.0) as u8, 
                             (locked_data[index as usize + 2] * 255.0) as u8]);
    }
    data_image.save("result.png").unwrap();

}
