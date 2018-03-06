use glium::Display;
use nalgebra::{zero, Vector2, Vector3};
use common::layout_group::MarginGroup;
use common::resource_group::{FontCacheGroup, FontGroup, ShaderGroup, TextureGroup};
use common::text_rendering::{layout_paragraph_cached, TextRenderError as RootTextRenderError};
use common::ui_primitive::{UiPrimitive, UiPrimitiveError as RootUiPrimitiveError};
use common::factory::ComponentFactory;
use components::model::Model;

/// A `UiElement` stands for a single object in the user interface. It may be composed of one or
/// more `UiPrimitive`s.
pub struct UiElement {
    pub model: Model,
    pub primitives: Vec<UiPrimitive>,
}

impl UiElement {
    pub fn new(model: Model, primitives: Vec<UiPrimitive>) -> Self {
        UiElement {
            model: model,
            primitives: primitives,
        }
    }
    pub fn create_textbox(
        display: &Display,
        factory: &mut ComponentFactory,
        cache: &mut FontCacheGroup,
        margin: &MarginGroup,
        font: &FontGroup,
        rect_shaders: &ShaderGroup,
        rect_textures: &TextureGroup,
        text_shaders: &ShaderGroup,
        position: &Vector3<f32>,
        offset: &Vector2<f32>,
        screen_dimensions: &Vector2<f32>,
        text_width: u32,
        text: &str,
    ) -> Result<Self, UiElementError> {
        // Layout the glyphs based on the textual `content`.
        let (glyphs, text_dims_px) = layout_paragraph_cached(
            &mut cache.cpu,
            &cache.gpu,
            &font.font,
            font.scale,
            text_width,
            text,
        )?;

        // Calculate positions and dimensions of the involved primitives: Text and Rect.
        let margin = margin.screen_to_ndc(screen_dimensions);
        let margin_sum = Vector2::new(margin.left + margin.right, margin.top + margin.bottom);

        let text_dims_ndc = Vector2::new(text_dims_px[0] as f32, text_dims_px[1] as f32)
            .component_div(screen_dimensions);
        let text_center = Vector2::new(-margin_sum.x, margin_sum.y) / 2.0
            + Vector2::new(margin.left, -margin.top);
        let text_center = Vector3::new(text_center.x, text_center.y, -0.01);

        let rect_dims_ndc = text_dims_ndc + margin_sum;
        let rect_center = Vector3::new(0.0, 0.0, 0.0);

        let element_center =
            Vector2::new(position.x, position.y) + offset.component_mul(&rect_dims_ndc);
        let element_center = Vector3::new(element_center.x, element_center.y, -0.98);

        let element_model = Model::new(element_center, zero(), Vector3::new(1.0, 1.0, 1.0));
        let rect = UiPrimitive::create_rectangle(
            display,
            factory,
            rect_center,
            rect_dims_ndc,
            rect_shaders,
            rect_textures,
        )?;
        let text = UiPrimitive::create_text(
            display,
            factory,
            &cache.cpu,
            screen_dimensions,
            text_center,
            &text_dims_ndc,
            &glyphs,
            text_shaders,
            font.color,
        )?;

        Ok(UiElement::new(element_model, vec![rect, text]))
    }
}

#[derive(Debug, Fail)]
pub enum UiElementError {
    #[fail(display = "{}", _0)] TextRenderError(#[cause] RootTextRenderError),
    #[fail(display = "{}", _0)] UiPrimitiveError(#[cause] RootUiPrimitiveError),
}

impl From<RootTextRenderError> for UiElementError {
    fn from(value: RootTextRenderError) -> Self {
        UiElementError::TextRenderError(value)
    }
}

impl From<RootUiPrimitiveError> for UiElementError {
    fn from(value: RootUiPrimitiveError) -> Self {
        UiElementError::UiPrimitiveError(value)
    }
}
