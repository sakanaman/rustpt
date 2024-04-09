
use crate::api::vector::Vector3;
use crate::api::material::Material;
use crate::api::material::SampleBSDF;
use crate::api::ray::Ray;
use crate::api::figure::Figure;
use crate::api::search::LinearSearch;
use crate::api::search::SearchAlgorithm;
use rand::rngs::ThreadRng;

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
        let material_info = 
        match shape {
            Figure::Sphere(s) => {
                match &(s.material) {
                    Material::Diffuse(m) => {
                        m.sample(&ray, &hitdata.normal, rng)
                    }
                }
            }
        };
        contribution = contribution * material_info.0;
        ray = material_info.1;
    }

    sky_color * contribution
}