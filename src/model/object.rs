use crate::model::ray::Ray;

// todo: rewrite to r.hits(o), for r: Ray; o: Some
pub trait Hit {
    fn hit(&self, ray: Ray) -> bool;
}

