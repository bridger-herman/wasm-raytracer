//! A generic object
//!
//! The only property we currently care about is if a ray intersects it.

use std::fmt;

use intersection::Intersection;
use material::Material;
use ray::Ray;

pub trait Object {
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;

    fn material(&self) -> &Material;

    fn info(&self) -> String;
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.info())
    }
}
