use crate::render::input::Input;
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use skia_safe::{Canvas, Color, ISize, Paint, Point, scalar};

pub enum Font {
    Regular(scalar, Color),
    Mono(scalar, Color),
}

pub trait ScreenRenderable {
    fn draw_text(
        &self,
        canvas: &Canvas,
        text: impl AsRef<str>,
        position: impl Into<Point>,
        font: Font,
        font_collection: &FontCollection,
    ) {
        let paragraph_style = ParagraphStyle::new();
        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

        let mut paint = Paint::default();
        let mut ts = TextStyle::new();
        paint.set_anti_alias(true);

        match font {
            Font::Regular(size, color) => {
                paint.set_color(color);
                ts.set_font_size(size);
            }
            Font::Mono(size, color) => {
                paint.set_color(color);
                ts.set_font_size(size);
                ts.set_font_families(&["JetBrains Mono"]);
            }
        }

        ts.set_foreground_paint(&paint);

        paragraph_builder.push_style(&ts);
        paragraph_builder.add_text(text);

        let mut paragraph = paragraph_builder.build();
        // TODO: make a prepare_text [returns &mut Paragraph] / draw_text [draws &Paragraph]
        paragraph.layout(256.0);
        paragraph.paint(canvas, position);
    }

    fn render(
        &self,
        canvas: &Canvas,
        input: &Input,
        screen_size: &ISize,
        font_collection: &FontCollection,
    );
}
