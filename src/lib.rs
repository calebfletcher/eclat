pub mod camera;
pub mod colour;
pub mod line;
pub mod mesh;
pub mod perspective;
pub mod pipeline;
pub mod triangle;

use std::path::Path;

use camera::Camera;
use colour::*;
use glam::{Mat4, Vec3, Vec4Swizzles};
use image::Rgb;
use mesh::Mesh;
use perspective::Perspective;
use triangle::Triangle;

pub struct PixelBuffer<'a> {
    buffer: &'a mut [u32],
    width: usize,
    height: usize,
}

impl<'a> PixelBuffer<'a> {
    pub fn new(buffer: &'a mut [u32], width: usize, height: usize) -> Self {
        Self {
            buffer,
            width,
            height,
        }
    }

    pub fn clear(&mut self, colour: Colour) {
        self.buffer.fill(colour.into());
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, colour: Colour) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.buffer[self.index(x, y)] = colour.into();
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn save_buffer(&self, path: impl AsRef<Path>) {
        let pixels = self
            .buffer
            .iter()
            .flat_map(|&pixel| {
                let pixel = Colour::from(pixel);
                [pixel.r, pixel.g, pixel.b]
            })
            .collect::<Vec<_>>();
        let img = image::ImageBuffer::<Rgb<u8>, _>::from_vec(
            self.width as u32,
            self.height as u32,
            pixels,
        )
        .unwrap();
        img.save(path).unwrap();
    }

    pub fn line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, colour: Colour) {
        let (mut x0, mut y0, x1, y1) = (x0 as i32, y0 as i32, x1 as i32, y1 as i32);
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;

        loop {
            self.set_pixel(x0 as usize, y0 as usize, colour);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 {
                    break;
                }
                error += dy;
                x0 += sx;
            }
            if e2 <= dx {
                if y0 == y1 {
                    break;
                }
                error += dx;
                y0 += sy;
            }
        }
    }

    pub fn triangle(&mut self, triangle: Triangle, colours: Option<[Colour; 3]>) {
        let aabb = triangle.aabb();

        for x in aabb.top_left.x as usize..=aabb.bottom_right.x as usize {
            for y in aabb.top_left.y as usize..=aabb.bottom_right.y as usize {
                let bary = triangle.barycentric(x as f32, y as f32);
                if bary.cmpge(Vec3::ZERO).all() && bary.cmple(Vec3::ONE).all() {
                    let colour = if let Some(colours) = colours {
                        Colour::new(
                            (bary.x * colours[0].r as f32
                                + bary.y * colours[1].r as f32
                                + bary.z * colours[2].r as f32)
                                .round() as u8,
                            (bary.x * colours[0].g as f32
                                + bary.y * colours[1].g as f32
                                + bary.z * colours[2].g as f32)
                                .round() as u8,
                            (bary.x * colours[0].b as f32
                                + bary.y * colours[1].b as f32
                                + bary.z * colours[2].b as f32)
                                .round() as u8,
                        )
                    } else {
                        Colour::new(
                            (bary.x * 255.) as u8,
                            (bary.y * 255.) as u8,
                            (bary.z * 255.) as u8,
                        )
                    };
                    self.set_pixel(x, y, colour);
                }
            }
        }
    }

    pub fn mesh(&mut self, mesh: Mesh, camera: Camera, proj: Perspective) {
        let ndc_to_ss = self.ndc_to_ss();
        let view_matrix = camera.as_matrix();
        let proj_matrix = proj.as_matrix();
        let viewproj = proj_matrix * view_matrix;

        for indices in mesh.indices.chunks_exact(3) {
            let p1 = mesh.vertices[indices[0]];
            let p2 = mesh.vertices[indices[1]];
            let p3 = mesh.vertices[indices[2]];
            let c1 = mesh.colours[indices[0]];
            let c2 = mesh.colours[indices[1]];
            let c3 = mesh.colours[indices[2]];

            let p1_clip = viewproj * p1.extend(1.);
            let p2_clip = viewproj * p2.extend(1.);
            let p3_clip = viewproj * p3.extend(1.);

            let p1_ndc = p1_clip / p1_clip.w;
            let p2_ndc = p2_clip / p2_clip.w;
            let p3_ndc = p3_clip / p3_clip.w;

            // Convert NDC to screen-space coordinates
            let p1_ss = (ndc_to_ss * p1_ndc).xy();
            let p2_ss = (ndc_to_ss * p2_ndc).xy();
            let p3_ss = (ndc_to_ss * p3_ndc).xy();

            let is_front_face = (p2_ndc.truncate() - p1_ndc.truncate())
                .cross(p3_ndc.truncate() - p1_ndc.truncate())
                .z
                < 0.;

            if is_front_face {
                self.triangle(Triangle::new(p1_ss, p2_ss, p3_ss), Some([c1, c2, c3]));
            }
        }
    }

    fn ndc_to_ss(&self) -> Mat4 {
        let halfwidth = self.width as f32 / 2.;
        let halfheight = self.height as f32 / 2.;
        Mat4::from_cols_array_2d(&[
            [halfwidth, 0., 0., halfwidth],
            [0., halfheight, 0., halfheight],
            [0., 0., 1., 0.], // 1 is to preserve depth
            [0., 0., 0., 0.],
        ])
        .transpose()
    }
}
