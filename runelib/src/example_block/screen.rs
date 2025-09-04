use crate::render::input::{Input, KeyState, MouseButton};
use crate::screen::{Font, ScreenRenderable, ScreenRenderableExt};
use skia_safe::textlayout::FontCollection;
use skia_safe::{Canvas, Color, ISize, Paint, Rect};

#[derive(Default)]
pub struct ExampleBlockScreen {
    editor_rect: Rect,
    editor_size: i32,
    text_box: String,
}

impl ScreenRenderable for ExampleBlockScreen {
    fn render(
        &mut self,
        canvas: &Canvas,
        input: &Input,
        screen_size: &ISize,
        font_collection: &FontCollection,
    ) {

        let textarea = Rect::new(10.0,10.0,110.0,50.0);
        let mut p =Paint::default();
        p.set_color(Color::WHITE);
        canvas.draw_rect(textarea, &p);

        if input.is_mouse_hovering(textarea){
            if let Some(key_state) = input.key_state{
                self.text_box+=key_state.;
            }
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
            canvas,
            "yo whats up -> === !=",
            (screen_size.width / 2, screen_size.height / 2),
            Font::Mono(32.0, Color::BLACK),
            font_collection,
        );
    }
}
