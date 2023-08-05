use glam::{vec3, Vec3};

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

    pub fn plane() -> Self {
        let vertices = vec![
            vec3(-1., -1., 0.),
            vec3(-1., 1., 0.),
            vec3(1., -1., 0.),
            vec3(1., 1., 0.),
        ];
        let indices = vec![0, 1, 2, 1, 2, 3];
        let colours = [Colour::RED, Colour::GREEN, Colour::BLUE, Colour::WHITE]
            .into_iter()
            .cycle()
            .take(vertices.len())
            .collect();

        Mesh::new(vertices, indices, colours)
    }

    pub fn cube() -> Self {
        let vertices = vec![
            vec3(-1., -1., -1.),
            vec3(-1., -1., 1.),
            vec3(-1., 1., -1.),
            vec3(-1., 1., 1.),
            vec3(1., -1., -1.),
            vec3(1., -1., 1.),
            vec3(1., 1., -1.),
            vec3(1., 1., 1.),
        ];
        let indices = vec![
            0, 1, 2, 1, 2, 3, 0, 1, 4, 1, 4, 5, 2, 3, 6, 3, 6, 7, 4, 5, 6, 5, 6, 7,
        ];
        let colours = [Colour::RED, Colour::GREEN, Colour::BLUE, Colour::WHITE]
            .into_iter()
            .cycle()
            .take(vertices.len())
            .collect();

        Mesh::new(vertices, indices, colours)
    }
}
