use crate::example_block::jni::{ExampleBlockRenderData, UIScript};
use crate::render::input::{Input, KeyState, MouseButton};
use crate::screen::text_input::TextInput;
use crate::screen::{DrawContext, Font, ScreenRenderable, ScreenRenderableExt};
use skia_safe::textlayout::{FontCollection, ParagraphStyle};
use skia_safe::{Canvas, Color, ISize, Paint, Rect};
use std::any::Any;
use std::path::PathBuf;

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
        render_data: &mut ExampleBlockRenderData,
    ) {
        let context = DrawContext::new(canvas, input, font_collection);

        if let Some(input) = &self.text_input {
            let new_path = PathBuf::from(&input.text);
            if render_data.target_directory != new_path {
                render_data.target_directory = new_path;
                // ignore the error for now .. we dont superrrr care
                let _ = render_data.collect_directory();
            }
        }

        if self.text_input.is_none() {
            self.text_input = Some(TextInput::new(
                (100, 400).into(),
                Font::Mono(16.0, Color::WHITE),
                Some(300),
            ));
        }

        if let Some(text_input) = &mut self.text_input {
            text_input.render(canvas, input, screen_size, font_collection, &mut ());
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

        render_data
            .scripts
            .iter_mut()
            .enumerate()
            .for_each(|(index, (script))| {
                let font = Font::Mono(16.0, Color::WHITE);
                if let Some(text_bounds) =
                    font.measure_text_bounds(&script.file_name, Some(&paint), font_collection)
                {
                    let render_pos = (
                        screen_size.width / 2,
                        100 + screen_size.height / 2 + (index as i32 * 10),
                    );
                    let target = text_bounds.with_offset(render_pos);

                    paint.set_color(Color::from_argb(255, 255, 255, 255));

                    if input.is_mouse_hovering(target) {
                        paint.set_color(Color::from_argb(255, 255, 0, 255));

                        if input.is_mouse_down(MouseButton::Left) {
                            render_data.active_script = Some(script.clone());
                        }
                    }

                    if let Some(active_script) = &render_data.active_script
                        && script == active_script
                    {
                        paint.set_color(Color::from_argb(255, 0, 0, 255));
                    }

                    self.draw_text(
                        &context,
                        &script.file_name,
                        render_pos,
                        &Font::Mono(16.0, paint.color()),
                    );
                }
            });

        if let Some(active_script) = &render_data.active_script
        {
            self.draw_text(&context, &active_script.content, (0.0, 0.0), &Font::Mono(14.0, Color::GRAY));
        }
    }
}
