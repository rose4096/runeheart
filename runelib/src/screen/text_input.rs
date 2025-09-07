use crate::render::input::{Input, KeyState, MouseButton};
use crate::screen::{DrawContext, Font, ScreenRenderable, ScreenRenderableExt};
use skia_safe::textlayout::{FontCollection, ParagraphStyle, RectHeightStyle, RectWidthStyle};
use skia_safe::{Canvas, Color, ISize, Paint, Point, Rect};
use std::ops::Range;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct TextSelection {
    anchor: usize,
    range: Range<usize>,
}

impl TextSelection {
    pub fn new(anchor: usize) -> Self {
        Self {
            anchor,
            range: anchor..anchor,
        }
    }

    pub fn with_start(mut self, start: usize) -> Self {
        self.range = start..self.anchor;
        self
    }

    pub fn with_end(mut self, end: usize) -> Self {
        self.range = self.anchor..end;
        self
    }

    pub fn set_end(&mut self, end: usize) -> &mut Self {
        self.range.end = end;
        self
    }

    pub fn set_start(&mut self, start: usize) -> &mut Self {
        self.range.start = start;
        self
    }

    pub fn shift(&mut self, direction: Direction, dest: usize) {
        match direction {
            Direction::Left => {
                if self.range.end > self.anchor {
                    self.set_end(self.range.end - 1);
                } else {
                    self.set_start(dest);
                }
            }
            Direction::Right => {
                if self.range.start < self.anchor {
                    self.set_start(self.range.start + 1);
                } else {
                    self.set_end(dest);
                }
            }
            _ => {}
        }
    }
}

trait OptionTextSelectionExt {
    fn select(&mut self, direction: Direction, cursor: usize, destination: usize);
}

impl OptionTextSelectionExt for Option<TextSelection> {
    fn select(&mut self, direction: Direction, cursor: usize, destination: usize) {
        match direction {
            Direction::Left => {
                self.get_or_insert_with(|| TextSelection::new(cursor).with_start(destination))
                    .shift(direction, destination);
            }
            Direction::Right => {
                self.get_or_insert_with(|| TextSelection::new(cursor).with_end(destination))
                    .shift(direction, destination);
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct TextInput {
    position: Point,
    text: String,
    focused: bool,
    font: Font,
    max_width: Option<i32>,
    cursor_pos: usize,
    selected_text: Option<TextSelection>,
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
            selected_text: None,
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

        rects.last().map_or(0.0, |x| x.rect.right)
    }

    fn selection_position(
        &self,
        paragraph: &skia_safe::textlayout::Paragraph,
    ) -> Option<(f32, f32)> {
        let selection_range = self.selected_text.as_ref()?;
        let range = if selection_range.range.start > selection_range.range.end {
            selection_range.range.end..selection_range.range.start
        } else {
            selection_range.range.start..selection_range.range.end
        };

        if range.start > self.text.len() || range.end > self.text.len() {
            return None;
        }

        let utf16_start_pos = self.text[..range.start].encode_utf16().count();
        let utf16_end_pos = self.text[..range.end].encode_utf16().count();

        let rects = paragraph.get_rects_for_range(
            utf16_start_pos..utf16_end_pos,
            RectHeightStyle::Tight,
            RectWidthStyle::Tight,
        );

        Some((rects.first()?.rect.left, rects.last()?.rect.right))
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
                bottom: self.position.y + height,
            };

            if input.is_mouse_hovering(rect) {
                paint.set_color(Color::LIGHT_GRAY);

                if input.is_mouse_down(MouseButton::Left) {
                    self.focused = true;
                    self.cursor_pos = self.text.len();
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
                let paragraph_style = ParagraphStyle::new()
                    .set_max_lines(1)
                    .set_ellipsis("...")
                    .to_owned();
                let mut paragraph =
                    self.paragraph(&context, &self.text, &self.font, Some(paragraph_style));
                paragraph.layout(rect.width());
                self.draw_paragraph(&context, paragraph, self.position);
            } else {
                let paragraph_style = ParagraphStyle::new().set_max_lines(1).to_owned();
                let mut paragraph =
                    self.paragraph(&context, &self.text, &self.font, Some(paragraph_style));
                paragraph.layout(1_000_000.0);

                let caret_offset = self.caret_position(&paragraph);
                let overflow = (self.position.x + caret_offset) - (self.position.x + rect.width());
                let shift = if overflow > 0.0 { -overflow } else { 0.0 };

                if let Some((start, end)) = self.selection_position(&paragraph) {
                    canvas.draw_rect(
                        Rect::new(
                            self.position.x + start + shift,
                            self.position.y,
                            self.position.x + end + shift,
                            self.position.y + rect.height(),
                        ),
                        Paint::default().set_color(Color::MAGENTA),
                    );
                }

                canvas.draw_rect(
                    Rect::new(
                        self.position.x + caret_offset + shift,
                        self.position.y,
                        self.position.x + caret_offset + 1.0 + shift,
                        self.position.y + rect.height(),
                    ),
                    Paint::default().set_color(Color::YELLOW),
                );

                self.draw_paragraph(
                    &context,
                    paragraph,
                    (self.position.x + shift, self.position.y),
                );
            }
        }

        if self.focused && !input.typed_characters.is_empty() {
            input
                .typed_characters
                .iter()
                .filter_map(|character| std::char::from_u32(character.code_point as u32))
                .for_each(|ch| self.text.insert(self.cursor_pos, ch));

            self.cursor_pos = self
                .text
                .len()
                .min(self.cursor_pos + input.typed_characters.len());
        }

        const KEY_BACKSPACE: i32 = 259;
        const KEY_LEFT: i32 = 263;
        const KEY_RIGHT: i32 = 262;
        const SHIFT_MODIFIER: i32 = 1;
        const CTRL_MODIFIER: i32 = 2;
        const ALT_MODIFIER: i32 = 4;

        input.key_state.iter().find(|key| 'out: {
            match key {
                KeyState::Pressed(key) => match key.key_code {
                    KEY_LEFT => {
                        if key.modifiers == SHIFT_MODIFIER {
                            self.selected_text.select(
                                Direction::Left,
                                self.cursor_pos,
                                self.cursor_pos.saturating_sub(1),
                            );
                        } else {
                            self.selected_text = None;
                        }

                        self.cursor_pos = self.cursor_pos.saturating_sub(1);
                    }
                    KEY_RIGHT => {
                        if key.modifiers == SHIFT_MODIFIER {
                            self.selected_text.select(
                                Direction::Right,
                                self.cursor_pos,
                                self.text.len().min(self.cursor_pos + 1),
                            );
                        } else {
                            self.selected_text = None;
                        }

                        self.cursor_pos = self.text.len().min(self.cursor_pos + 1);
                    }
                    KEY_BACKSPACE => {
                        if !self.text.is_empty()
                            && let Some(cursor_pos) = self.cursor_pos.checked_sub(1)
                        {
                            self.cursor_pos = cursor_pos;
                            self.text.remove(self.cursor_pos);
                        }
                    }
                    _ => break 'out false,
                },
                KeyState::Released(_) => break 'out false,
            }
            true
        });
    }
}
