
use crate::api::vector::Vector3;
use crate::api::material::Material;
use crate::api::material::SampleBSDF;
use crate::api::ray::Ray;
use crate::api::figure::Figure;
use crate::api::search::LinearSearch;
use crate::api::search::SearchAlgorithm;
use rand::rngs::ThreadRng;
use rand::Rng;

const RUSSIAN_PROB : f64 = 0.9;

pub fn ray_trace(first_ray : Ray, searcher : &LinearSearch<Figure>, rng : &mut ThreadRng) -> Vector3{
    let mut contribution = Vector3{x:1.0, y:1.0, z:1.0};
    let sky_color = Vector3{x:1.0, y:1.0, z:1.0};
    let mut ray = first_ray;

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
        let material_info = 
        match shape {
            Figure::Sphere(s) => {
                if s.emission.x > 0.0 || s.emission.y > 0.0 || s.emission.z > 0.0
                {
                    contribution = &contribution * &s.emission;
                    is_light = true;
                } 

                match &(s.material) {
                    Material::Diffuse(m) => {
                        m.sample(&ray, &hitdata.normal, rng)
                    }
                }
            }
        };

        if is_light == true {
           return contribution;
        }

        if RUSSIAN_PROB < rng.gen() {
            contribution = Vector3{x:0.0, y:0.0, z:0.0};
            break;
        }

        contribution = contribution * material_info.0 * (1.0 / RUSSIAN_PROB);
        ray = material_info.1;
    }

    sky_color * contribution
}