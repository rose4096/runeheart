use crate::render::input::{Input, MouseButton, Position};
use crate::screen::{DrawContext, Font, ScreenRenderable, ScreenRenderableExt};
use skia_safe::textlayout::FontCollection;
use skia_safe::{Canvas, Color, ISize, Paint, Point, Rect, Size};

#[derive(Debug)]
pub struct TextInput {
    position: Point,
    text: String,
    focused: bool,
    font: Font,
    max_width: Option<i32>,
}

impl TextInput {
    pub fn new(position: Point, font: Font, max_width: Option<i32>) -> Self {
        Self {
            position,
            font,
            max_width,
            text: String::default(),
            focused: false,
        }
    }

    fn text(&self) -> &str {
        &self.text
    }
}

impl ScreenRenderable for TextInput {
    fn render(
        &mut self,
        canvas: &Canvas,
        input: &Input,
        screen_size: &ISize,
        font_collection: &FontCollection,
    ) {
        let context = DrawContext::new(canvas, input, font_collection);

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(Color::BLACK);

        if let Some(height) = self.font.measure_height(font_collection) {
            println!("HEIGHT:{}", height);
            let rect = Rect {
                left: self.position.x,
                top: self.position.y,
                right: self.position.x + self.max_width.unwrap_or_default() as f32,
                bottom: self.position.y + height
            };

            println!("rect:{:?}",rect);
            println!("iimp:{:?}",input.mouse_position);
            if input.is_mouse_hovering(rect) {
                paint.set_color(Color::LIGHT_GRAY);

                if input.is_mouse_down(MouseButton::Left) {
                    self.focused = true;
                }
            } else if input.is_mouse_down(MouseButton::Left) {
                self.focused = false;
            }

            if self.focused {
                println!("focused");

                paint.set_color(Color::WHITE);
                context
                    .canvas
                    .draw_rect(rect.clone().with_outset((10, 10)), &paint);
                paint.set_color(Color::BLACK);
            }

            canvas.draw_rect(rect, &paint);

            // TODO: paragraph builder api with draw_text
            self.draw_text(&context, &self.text, self.position, &self.font);
        }

        if self.focused && !input.typed_characters.is_empty() {
            input.typed_characters.iter().for_each(|character| {
                match std::char::from_u32(character.code_point as u32) {
                    None => {}
                    Some(character) => self.text.push(character),
                }
            })
        }
    }
}
