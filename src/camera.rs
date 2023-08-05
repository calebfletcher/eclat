use glam::{Mat4, Vec3};

pub struct Camera {
    position: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
}

impl Camera {
    pub fn looking_at(position: Vec3, target: Vec3) -> Self {
        let forward = (target - position).normalize();
        let up = Vec3::Y;

        let right = forward.cross(up).normalize();
        let up = right.cross(forward);

        Self {
            position,
            forward,
            right,
            up,
        }
    }

    pub fn as_matrix(&self) -> Mat4 {
        let t = self.position;
        let r = self.right;
        let f = self.forward;
        let u = self.up;

        let a = Mat4::from_cols_array_2d(&[
            [r.x, r.y, r.z, 0.],
            [u.x, u.y, u.z, 0.],
            [f.x, f.y, f.z, 0.],
            [0., 0., 0., 1.],
        ])
        .transpose();

        let b = Mat4::from_cols_array_2d(&[
            [1., 0., 0., -t.x],
            [0., 1., 0., -t.y],
            [0., 0., 1., -t.z],
            [0., 0., 0., 1.],
        ])
        .transpose();

        a * b
    }
}
