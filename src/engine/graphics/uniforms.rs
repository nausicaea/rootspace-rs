use glium::uniforms;

pub struct Uniforms {
}

impl uniforms::Uniforms for Uniforms {
    fn visit_values<'a, F>(&'a self, f: F) where F: FnMut(&str, uniforms::UniformValue<'a>) {
        // f("name", uniform_value);
    }
}
