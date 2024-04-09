use crate::api::vector::INF_DISTANCE;
use crate::api::ray::Ray;
use crate::api::figure::Figure;
use crate::api::figure::Hit;
use crate::api::figure::Intersect;

pub trait SearchAlgorithm<T> {
    fn add(&mut self, figure : T);
    fn search(&self, ray : &Ray)->Option<Hit>;
    fn get_shape(&self, hitdata : &Hit)->&Figure;
}

pub struct LinearSearch<T> {
    pub figures : Vec<T> 
}

impl SearchAlgorithm<Figure> for LinearSearch<Figure> {
    fn add(&mut self, figure : Figure) {
        self.figures.push(figure);
    }

    fn search(&self, ray : &Ray)->Option<Hit>{
        let mut min_dist = INF_DISTANCE;
        let mut hitdata_result : Option<Hit> = None;
        let mut counter : usize = 0;
        for figure in &self.figures {
            let hitdata_candidate = match figure {
                Figure::Sphere(s) => {
                   s.intersect(ray, &counter)
                }
            };

            match hitdata_candidate {
                Some(s) => {
                    if s.distance < min_dist {
                        min_dist = s.distance;
                        hitdata_result = Some(s);
                    } 
                },
                None => ()
            }
            counter += 1;
        }
        hitdata_result
    }

    fn get_shape(&self, hitdata : &Hit)->&Figure
    {
        let object_id = hitdata.object_id;
        &self.figures[object_id]
    }
}