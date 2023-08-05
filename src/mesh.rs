use glam::Vec3;

use crate::Colour;

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<usize>,
    pub colours: Vec<Colour>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vec3>, indices: Vec<usize>, colours: Vec<Colour>) -> Self {
        Self {
            vertices,
            indices,
            colours,
        }
    }
}
