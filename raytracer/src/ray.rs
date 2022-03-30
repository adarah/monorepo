use crate::vec3::{Point3, Vec3};
pub struct Ray<'a> {
    pub origin: &'a Point3,
    pub direction: &'a Vec3
}

impl<'a> Ray<'a> {

    pub fn new(origin: &'a Point3, direction: &'a Vec3) -> Self {
        Self{origin, direction}
    }
    
    pub fn at(self, time: f64) -> Point3 {
        *self.origin + time * *self.direction
    }
}