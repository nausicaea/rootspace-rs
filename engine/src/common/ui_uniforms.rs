use nalgebra::{Matrix4, Vector3};
use glium::uniforms;
use glium::texture::Texture2d;

pub struct UiUniforms<'t> {
    pub pvm_matrix: Matrix4<f32>,
    pub font_cache: &'t Texture2d,
    pub font_color: Vector3<f32>,
    pub diff_tex: Option<&'t Texture2d>,
    pub norm_tex: Option<&'t Texture2d>,
}

impl<'t> uniforms::Uniforms for UiUniforms<'t> {
    fn visit_values<'a, F>(&'a self, mut f: F)
    where
        F: FnMut(&str, uniforms::UniformValue<'a>),
    {
        f(
            "pvm_matrix",
            uniforms::UniformValue::Mat4(self.pvm_matrix.into()),
        );
        f(
            "font_cache",
            uniforms::UniformValue::Texture2d(self.font_cache, None),
        );
        f(
            "font_color",
            uniforms::UniformValue::Vec3(self.font_color.into()),
        );
        if let Some(t) = self.diff_tex {
            f("diff_tex", uniforms::UniformValue::Texture2d(t, None));
        }
        if let Some(t) = self.norm_tex {
            f("norm_tex", uniforms::UniformValue::Texture2d(t, None));
        }
    }
}
