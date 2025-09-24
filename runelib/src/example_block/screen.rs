use std::any::Any;
use crate::render::input::{Input, KeyState, MouseButton};
use crate::screen::text_input::TextInput;
use crate::screen::{DrawContext, Font, ScreenRenderable, ScreenRenderableExt};
use skia_safe::textlayout::FontCollection;
use skia_safe::{Canvas, Color, ISize, Paint, Rect};
use crate::example_block::jni::ExampleBlockRenderData;

#[derive(Default)]
pub struct ExampleBlockScreen {
    editor_rect: Rect,
    editor_size: i32,
    text_input: Option<TextInput>,
    // TODO: create interpreter window + editor window + file list window
}

impl ScreenRenderable<ExampleBlockRenderData> for ExampleBlockScreen {
    fn render(
        &mut self,
        canvas: &Canvas,
        input: &Input,
        screen_size: &ISize,
        font_collection: &FontCollection,
        render_data: &ExampleBlockRenderData,
    ) {
        let context = DrawContext::new(canvas, input, font_collection);

        println!(" we won motherfucker {:?}", render_data);

        if self.text_input.is_none() {
            self.text_input = Some(TextInput::new(
                (100, 400).into(),
                Font::Mono(16.0, Color::WHITE),
                Some(300),
            ));
        }

        if let Some(text_input) = &mut self.text_input {
            text_input.render(canvas, input, screen_size, font_collection, &());
        }

        self.editor_rect = Rect::new(
            (screen_size.width / 2) as f32,
            0.0,
            screen_size.width as f32,
            screen_size.height as f32,
        );

        let mut paint = Paint::default();
        paint.set_color(Color::from_argb(50, 30, 30, 30));

        let bounded_rect = Rect {
            left: (self.editor_rect.left as i32 - self.editor_size) as f32,
            ..self.editor_rect
        };

        if input.is_mouse_hovering(Rect {
            left: bounded_rect.left - 10.0,
            right: bounded_rect.left + 10.0,
            ..bounded_rect
        }) {
            paint.set_color(Color::from_argb(255, 60, 255, 30));

            if input.is_mouse_down(MouseButton::Left) {
                let offset = (screen_size.width / 2) - input.mouse_position.x;
                self.editor_size = offset.clamp(-(screen_size.width / 6), screen_size.width / 6);
            }
        }

        canvas.draw_rect(bounded_rect, &paint);

        self.draw_text(
            &context,
            "yo whats up -> === !=",
            (screen_size.width / 2, screen_size.height / 2),
            &Font::Mono(32.0, Color::BLACK),
        );
    }
}
