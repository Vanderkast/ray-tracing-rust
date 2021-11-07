use crate::model::ray::Ray;

pub trait Hit {
    ///
    fn hit(&self, ray: Ray) -> f32;
}

