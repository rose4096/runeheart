use skia_safe::{Canvas, Color, ISize};
use skia_safe::textlayout::FontCollection;
use crate::render::input::Input;
use crate::screen::{Font, ScreenRenderable};

#[derive(Default)]
pub struct ExampleBlockScreen {}

impl ScreenRenderable for ExampleBlockScreen {
    fn render(&self, canvas: &Canvas, input: &Input, screen_size: &ISize, font_collection: &FontCollection) {
        //resizablee ditor rect on right side

        self.draw_text(canvas, "yo whats up -> === !=", (screen_size.width / 2, screen_size.height / 2), Font::Mono(32.0, Color::BLACK), font_collection);
    }
}