use glam::Mat4;

pub struct Perspective {
    near: f32,
    far: f32,
    width: f32,
    height: f32,
}

impl Perspective {
    pub fn new(near: f32, far: f32, width: f32, height: f32) -> Self {
        Self {
            near,
            far,
            width,
            height,
        }
    }

    pub fn as_matrix(&self) -> Mat4 {
        let r = self.width / 2.;
        let t = self.height / 2.;
        let f = self.far;
        let n = self.near;

        Mat4::from_cols_array_2d(&[
            [n / r, 0., 0., 0.],
            [0., n / t, 0., 0.],
            [0., 0., -(f + n) / (f - n), -2. * f * n / (f - n)],
            [0., 0., -1., 0.],
        ])
        .transpose()
    }
}

#[cfg(test)]
mod tests {
    use glam::{vec3, vec4, Vec3, Vec4};

    use super::*;

    #[track_caller]
    fn check_proj(mat: Mat4, input: Vec3, expected: Vec4) {
        let clip = mat * input.extend(1.);
        let ndc = clip / clip.w;
        dbg!(clip, ndc);
        approx::assert_relative_eq!(ndc, expected);
    }

    #[test]
    fn perspective() {
        let proj = Perspective::new(0.1, 100., 10., 10.);
        let mat = proj.as_matrix();

        check_proj(mat, vec3(0., 0., -0.1), vec4(0., 0., -1., 1.));
        check_proj(mat, vec3(0., 0., -100.), vec4(0., 0., 1., 1.));
        check_proj(mat, vec3(5., 0., -0.1), vec4(1., 0., -1., 1.));
        check_proj(mat, vec3(-5., 0., -0.1), vec4(-1., 0., -1., 1.));
        check_proj(mat, vec3(0., 5., -0.1), vec4(0., 1., -1., 1.));
        check_proj(mat, vec3(0., -5., -0.1), vec4(0., -1., -1., 1.));
        check_proj(mat, vec3(5., 5., -0.1), vec4(1., 1., -1., 1.));
    }
}
