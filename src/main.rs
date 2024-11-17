mod api;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::thread::spawn;

use api::raytrace::ray_trace;
use api::{figure::Figure, material::Material, search::LinearSearch, vector::Vector3};
use api::material::Diffuse;

use crate::api::camera::Camera;
use crate::api::camera::new_edupt;
use crate::api::figure::Sphere;
use image;

fn clamp(val : f64, minimum : f64, maximum : f64) -> f64
{
    if val < minimum {
        return minimum;
    }
    else if val > maximum {
        return maximum; 
    }
    else {
        return val;
    }
}

fn make_image(data : Arc<Mutex<Vec<f64>>>, width : usize, height : usize, filename: String)
{
    let mut data_image = image::RgbImage::new(width.clone() as u32, height.clone() as u32);

    let locked_data = data.lock().unwrap();
    for (x, y, pixel) in data_image.enumerate_pixels_mut() {
        let index =  y*width as u32*3 + x*3;
        *pixel = image::Rgb([(clamp(locked_data[index as usize    ], 0.0, 1.0) * 255.0) as u8, 
                             (clamp(locked_data[index as usize + 1], 0.0, 1.0) * 255.0) as u8, 
                             (clamp(locked_data[index as usize + 2], 0.0, 1.0) * 255.0) as u8]);
    }
    data_image.save(filename).unwrap();
}

fn main() {
   let scene = 
    vec![
            Figure::Sphere(
                    Sphere{radius:1e5, 
                           position:Vector3{x:1.0+1e5,y:40.8,z:81.6},
                           emission:Vector3{x:0.,y:0.,z:0.}, 
                           material:Material::Diffuse(Diffuse{albedo: Vector3{x:0.75,y:0.25,z:0.25} })}
            ),
            Figure::Sphere(
                    Sphere{radius:1e5, 
                           position:Vector3{x:99.0-1e5,y:40.8,z:81.6},
                           emission:Vector3{x:0.,y:0.,z:0.}, 
                           material:Material::Diffuse(Diffuse{albedo: Vector3{x:0.25,y:0.25,z:0.75} })}
            ),
            Figure::Sphere(
                    Sphere{radius:1e5, 
                           position:Vector3{x:50.0,y:40.8,z:1e5},
                           emission:Vector3{x:0.,y:0.,z:0.}, 
                           material:Material::Diffuse(Diffuse{albedo: Vector3{x:0.75,y:0.75,z:0.75} })}
            ),
            // Figure::Sphere(
            //         Sphere{radius:1e5, 
            //                position:Vector3{x:50.0,y:40.8,z:250.0-1e5},
            //                emission:Vector3{x:0.,y:0.,z:0.}, 
            //                material:Material::Diffuse(Diffuse{albedo: Vector3{x:0.99,y:0.99,z:0.99} })}
            // ),
            Figure::Sphere(
                    Sphere{radius:1e5, 
                           position:Vector3{x:50.0,y:1e5,z:81.6},
                           emission:Vector3{x:0.0,y:0.0,z:0.0}, 
                           material:Material::Diffuse(Diffuse{albedo: Vector3{x:0.75,y:0.75,z:0.75} })}
            ),
            Figure::Sphere(
                    Sphere{radius:1e5, 
                           position:Vector3{x:50.0,y:81.6-1e5,z:81.6},
                           emission:Vector3{x:0.,y:0.,z:0.}, 
                           material:Material::Diffuse(Diffuse{albedo: Vector3{x:0.75,y:0.75,z:0.75} })}
            ),
            // Figure::Sphere(
            //         Sphere{radius:20.0, 
            //                position:Vector3{x:65.0,y:20.0,z:81.6},
            //                emission:Vector3{x:0.,y:0.,z:0.}, 
            //                material:Material::Diffuse(Diffuse{albedo: Vector3{x:0.25,y:0.75,z:0.25} })}
            // ),
            Figure::Sphere(
                    Sphere{radius:16.5, 
                           position:Vector3{x:27.0,y:16.5,z:78.0},
                           emission:Vector3{x:0.0,y:0.0,z:0.0}, 
                           material:Material::Diffuse(Diffuse{albedo: Vector3{x:0.25,y:0.99,z:0.99} })}
            ),
            Figure::Sphere(
                    Sphere{radius:16.5, 
                           position:Vector3{x:77.0,y:16.5,z:78.0},
                           emission:Vector3{x:0.0,y:0.0,z:0.0}, 
                           material:Material::Diffuse(Diffuse{albedo: Vector3{x:0.99,y:0.25,z:0.99} })}
            ),
            Figure::Sphere(
                    Sphere{radius:15.0, 
                           position:Vector3{x:50.0,y:90.0,z:78.0},
                           emission:Vector3{x:36.,y:36.,z:36.}, 
                           material:Material::Diffuse(Diffuse{albedo: Vector3{x:0.99,y:0.99,z:0.99} })}
            ),
        ];
    let searcher = Arc::new(LinearSearch{figures:scene});
    let width = 252;
    let height = 252;
    // let aspect_ratio = width as f64 / height as f64;
    let spp = 1000;
    const THREAD_NUM : i32 = 8;

    let data = Arc::new(Mutex::new(vec![0.0_f64;height*width*3]));

    let mut thrd:Vec<JoinHandle<()>> = Vec::new();

    let pcam = Arc::new(new_edupt(width, height));

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
                if thread_id == 0 {
                    println!("{}", sample_id)
                } 
            }
        };
        thrd.push(spawn(task));
    }
    thrd.into_iter().for_each(|th| {
        let _ = th.join();
    });

    let filename = "result.png".to_string();
    make_image(data, width, height, filename);

}
