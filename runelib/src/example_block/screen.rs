use crate::render::input::{Input, MouseButton};
use crate::screen::{Font, ScreenRenderable};
use skia_safe::textlayout::FontCollection;
use skia_safe::{Canvas, Color, ISize, Paint, Rect};

#[derive(Default)]
pub struct ExampleBlockScreen {}

impl ScreenRenderable for ExampleBlockScreen {
    fn render(
        &self,
        canvas: &Canvas,
        input: &Input,
        screen_size: &ISize,
        font_collection: &FontCollection,
    ) {
        //resizablee ditor rect on right side
        let mut editor_rect = Rect::new(
            (screen_size.width / 2) as f32,
            0.0,
            screen_size.width as f32,
            screen_size.height as f32,
        );

        if let Some(down) = &input.mouse_button_down && *down == MouseButton::Left {
            let original = editor_rect.left;
            editor_rect.left = input.mouse_position.x as f32;
            if editor_rect.left < (screen_size.width / 3) as f32 {
                editor_rect.left = original;
            }
        }

        let mut paint = Paint::default();
        paint.set_color(Color::from_argb(50, 30, 30, 30));

        if input.mouse_position.x > (editor_rect.left - 10.0) as i32
            && input.mouse_position.x < (editor_rect.left + 10.0) as i32
            && input.mouse_position.y > editor_rect.top as i32
            && input.mouse_position.y < editor_rect.bottom as i32
        {
            paint.set_color(Color::from_argb(255, 60, 255, 30));
        }

        canvas.draw_rect(editor_rect, &paint);

        self.draw_text(
            canvas,
            "yo whats up -> === !=",
            (screen_size.width / 2, screen_size.height / 2),
            Font::Mono(32.0, Color::BLACK),
            font_collection,
        );
    }
}
