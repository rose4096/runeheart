use std::ops::Add;
use crate::render::input::{Input, MouseButton, Position};
use crate::screen::{DrawContext, Font, ScreenRenderable, ScreenRenderableExt};
use skia_safe::textlayout::{FontCollection, ParagraphStyle, RectHeightStyle, RectWidthStyle};
use skia_safe::{Canvas, Color, ISize, Paint, Point, Rect, Size};

#[derive(Debug)]
pub struct TextInput {
    position: Point,
    text: String,
    focused: bool,
    font: Font,
    max_width: Option<i32>,
    cursor_pos: usize,
    scroll_x: f32,       // how far we've scrolled horizontally
}

impl TextInput {
    pub fn new(position: Point, font: Font, max_width: Option<i32>) -> Self {
        Self {
            position,
            font,
            max_width,
            text: String::default(),
            focused: false,
            cursor_pos: 0,
            scroll_x: 0.0
        }
    }

    fn text(&self) -> &str {
        &self.text
    }

    fn caret_position(&self, paragraph: &skia_safe::textlayout::Paragraph) -> f32 {
        let rects = paragraph.get_rects_for_range(
            0..self.text[..self.cursor_pos].encode_utf16().count(),
            RectHeightStyle::Tight,
            RectWidthStyle::Tight,
        );

        println!("{:?}", rects);
        if let Some(r) = rects.last() {
            r.rect.right
        } else {
            0.0
        }
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
            let rect = Rect {
                left: self.position.x,
                top: self.position.y,
                right: self.position.x + self.max_width.unwrap_or_default() as f32,
                bottom: self.position.y + height
            };

            if input.is_mouse_hovering(rect) {
                paint.set_color(Color::LIGHT_GRAY);

                if input.is_mouse_down(MouseButton::Left) {
                    self.focused = true;
                }
            } else if input.is_mouse_down(MouseButton::Left) {
                self.focused = false;
            }

            if self.focused {
                paint.set_color(Color::WHITE);
                context
                    .canvas
                    .draw_rect(rect.clone().with_outset((2, 2)), &paint);
                paint.set_color(Color::BLACK);
            }

            canvas.draw_rect(rect, &paint);

            if !self.focused {
                let paragraph_style = ParagraphStyle::new().set_max_lines(1).set_ellipsis("...").to_owned();
                let mut paragraph = self.paragraph(&context, &self.text, &self.font, Some(paragraph_style));
                paragraph.layout(rect.width());
                self.draw_paragraph(&context, paragraph, self.position);
            } else {
                let paragraph_style = ParagraphStyle::new().set_max_lines(1).to_owned();
                let mut paragraph = self.paragraph(&context, &self.text, &self.font, Some(paragraph_style));
                paragraph.layout(1_000_000.0);
                let caret_x = self.caret_position(&paragraph);
                if caret_x - self.scroll_x > rect.width() {
                    self.scroll_x = caret_x - rect.width();
                }
                let draw_x = self.position.x - self.scroll_x;
                self.draw_paragraph(&context, paragraph, (draw_x, self.position.y));
            }
        }

        if self.focused && !input.typed_characters.is_empty() {
            input.typed_characters.iter().for_each(|character| {
                match std::char::from_u32(character.code_point as u32) {
                    None => {}
                    Some(character) => self.text.push(character),
                }
            });
            self.cursor_pos = self.text.len();
        }

        if let Some(key) = &input.key_state {
            const KEY_BACKSPACE: i32 = 259;
            const KEY_LEFT: i32 = 263;
            const KEY_RIGHT: i32 = 262;

            // TODO: represent key press state betetr (tyeps for releas,epress,etc.
            match key.key_code {
                KEY_LEFT => {
                    self.cursor_pos = 0.max(self.cursor_pos -1);
                },
                KEY_RIGHT => {
                    self.cursor_pos = self.text.len().min(self.cursor_pos + 1);
                },
                KEY_BACKSPACE => {
                    self.text.pop();
                    self.cursor_pos = self.text.len();
                },
                _ => {}
            }
        }
    }
}
