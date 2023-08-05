use glam::{vec2, vec3, Vec2, Vec3};

#[derive(Debug, Clone)]
pub struct Triangle {
    p1: Vec2,
    p2: Vec2,
    p3: Vec2,
}

impl Triangle {
    pub fn new(p1: Vec2, p2: Vec2, p3: Vec2) -> Self {
        Self { p1, p2, p3 }
    }

    pub fn barycentric(&self, x: f32, y: f32) -> Vec3 {
        let denom = (self.p2.y - self.p3.y) * (self.p1.x - self.p3.x)
            + (self.p3.x - self.p2.x) * (self.p1.y - self.p3.y);
        let lambda1 = ((self.p2.y - self.p3.y) * (x - self.p3.x)
            + (self.p3.x - self.p2.x) * (y - self.p3.y))
            / denom;
        let lambda2 = ((self.p3.y - self.p1.y) * (x - self.p3.x)
            + (self.p1.x - self.p3.x) * (y - self.p3.y))
            / denom;
        let lambda3 = 1. - lambda1 - lambda2;

        vec3(lambda1, lambda2, lambda3)
    }

    pub fn aabb(&self) -> Aabb {
        let min_x = self.p1.x.min(self.p2.x).min(self.p3.x).floor();
        let max_x = self.p1.x.max(self.p2.x).max(self.p3.x).ceil();
        let min_y = self.p1.y.min(self.p2.y).min(self.p3.y).floor();
        let max_y = self.p1.y.max(self.p2.y).max(self.p3.y).ceil();

        Aabb {
            top_left: vec2(min_x, min_y),
            bottom_right: vec2(max_x, max_y),
        }
    }
}

pub struct Aabb {
    pub top_left: Vec2,
    pub bottom_right: Vec2,
}
