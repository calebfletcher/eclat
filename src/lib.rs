mod colour;
mod line;
mod pipeline;
mod triangle;

pub use colour::*;
use glam::Vec3;
pub use triangle::Triangle;

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

    pub fn triangle(&mut self, triangle: Triangle) {
        let aabb = triangle.aabb();

        for x in aabb.top_left.x as usize..=aabb.bottom_right.x as usize {
            for y in aabb.top_left.y as usize..=aabb.bottom_right.y as usize {
                let bary = triangle.barycentric(x as f32, y as f32);
                const AA_DIST: f32 = 0.01;
                let min = bary.min_element();
                if bary.cmpge(Vec3::ZERO).all() && bary.cmple(Vec3::ONE).all() {
                    self.set_pixel(
                        x,
                        y,
                        Colour::new(
                            (bary.x * 255.) as u8,
                            (bary.y * 255.) as u8,
                            (bary.z * 255.) as u8,
                        ),
                    );
                } else if min < 0. && min > -AA_DIST {
                    let alpha = 1. + min / AA_DIST;
                    self.set_pixel(
                        x,
                        y,
                        alpha
                            * Colour::new(
                                (bary.x * 255.) as u8,
                                (bary.y * 255.) as u8,
                                (bary.z * 255.) as u8,
                            ),
                    );
                }
            }
        }
    }
}