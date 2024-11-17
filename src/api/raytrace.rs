
use crate::api::vector::Vector3;
use crate::api::material::Material;
use crate::api::material::SampleBSDF;
use crate::api::ray::Ray;
use crate::api::figure::Figure;
use crate::api::search::LinearSearch;
use crate::api::search::SearchAlgorithm;
use rand::rngs::ThreadRng;
use rand::Rng;

fn sky_light(ray: &Ray)->Vector3
{
    let t = 0.5 * (ray.dir.y + 1.0);
    return (1.0 - t) * Vector3{x:1.0, y:1.0, z:1.0} + t * Vector3{x:0.5, y:0.7, z:1.0};
    // Vector3{x:1.0, y:1.0, z:1.0}
    // Vector3{x:0.0, y:0.0, z:0.0}
}

pub fn ray_trace(first_ray : Ray, searcher : &LinearSearch<Figure>, rng : &mut ThreadRng) -> Vector3{
    let mut contribution = Vector3{x:1.0, y:1.0, z:1.0};
    let mut ray = first_ray;
    let mut pr : f64 = 1.0;
    loop {
        let wrapped_hitdata = searcher.search(&ray);
        let shape : &Figure;

        if let Some(s) = &wrapped_hitdata {
            shape = searcher.get_shape(s);
        }
        else {
            break; 
        }

       let hitdata = wrapped_hitdata.unwrap();

        // BSDF(or Volume) sample
        let mut is_light = false;
        let mut modified_normal = hitdata.normal.clone();
        let material_info =  // (weight, sample_dir)
        match shape {
            Figure::Sphere(s) => {
                if s.emission.x > 0.0 || s.emission.y > 0.0 || s.emission.z > 0.0
                {
                    contribution = &contribution * &s.emission;
                    is_light = true;
                } 

                if ray.dir.dot(&hitdata.normal) > 0.0 {
                    modified_normal = -1.0 * &hitdata.normal;
                }

                match &(s.material) {
                    Material::Diffuse(m) => {
                        m.sample(&ray, &modified_normal, rng)
                    }
                }
            }
        };
        // renew nextray
        ray = Ray{org:hitdata.position.clone(), dir:material_info.1};
        ray.org = &ray.org + &modified_normal ;

        if ray.dir.dot(&modified_normal) < 0.0 {
            println!("bad sample");
        }

        if is_light == true {
           return contribution;
        }

        if pr < rng.gen() {
            contribution = Vector3{x:0.0, y:0.0, z:0.0};
            break;
        }

        pr *= 0.96;
        contribution = contribution * material_info.0 * (1.0 / pr);
    }
    sky_light(&ray) * contribution
}