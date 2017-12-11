use nalgebra::Matrix4;
use glium::uniforms;
use super::super::Float;

pub struct Uniforms {
    pub pvm_matrix: Matrix4<Float>,
}

impl uniforms::Uniforms for Uniforms {
    fn visit_values<'a, F>(&'a self, mut f: F) where F: FnMut(&str, uniforms::UniformValue<'a>) {
        f("pvm_matrix", uniforms::UniformValue::Mat4(self.pvm_matrix.into()));
        // f("name", uniform_value);
        // I need: pvm_matrix, normal_matrix, optionally diff_tex, optionally norm_tex
    }
}
