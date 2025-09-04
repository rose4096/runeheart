pub mod text_input;

use crate::render::input::{Input, MouseButton};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use skia_safe::wrapper::PointerWrapper;
use skia_safe::{Canvas, Color, FontStyle, ISize, Paint, Point, Rect, Size, scalar};

#[derive(Debug)]
pub enum Font {
    Regular(scalar, Color),
    Mono(scalar, Color),
}

impl Font {
    fn measure_height(&self, font_collection: &FontCollection) -> Option<scalar> {
        let mut fc = font_collection.clone();
        // ^^ this sucks. but why is find_typefaces mutable??

        let families = match self {
            Font::Regular(_, _) => todo!(),
            Font::Mono(_, _) => &["JetBrains Mono"],
        };

        let size = match self {
            Font::Regular(sz, _) | Font::Mono(sz, _) => sz,
        };

        let tfs = fc.find_typefaces(families, FontStyle::normal());
        if let Some(tf) = tfs.first() {
            let font = skia_safe::Font::from_typeface(tf, *size);
            let metrics = font.metrics();
            // maybe use ascent/descent
            return Some(metrics.1.top.abs() + metrics.1.bottom.abs());
        }

        None
    }
}

pub struct DrawContext<'a> {
    canvas: &'a Canvas,
    input: &'a Input,
    font_collection: &'a FontCollection,
}

impl<'a> DrawContext<'a> {
    pub fn new(canvas: &'a Canvas, input: &'a Input, font_collection: &'a FontCollection) -> Self {
        Self {
            canvas,
            input,
            font_collection,
        }
    }
}

pub trait ScreenRenderable {
    fn draw_text_raw(&self, context: &DrawContext, text: &str, position: Point, font: &Font) {
        let paragraph_style = ParagraphStyle::new();
        let mut paragraph_builder =
            ParagraphBuilder::new(&paragraph_style, context.font_collection);

        let mut paint = Paint::default();
        let mut ts = TextStyle::new();
        paint.set_anti_alias(true);

        match *font {
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
        paragraph.paint(context.canvas, position);
    }

    fn render(
        &mut self,
        canvas: &Canvas,
        input: &Input,
        screen_size: &ISize,
        font_collection: &FontCollection,
    );
}

pub trait ScreenRenderableExt: ScreenRenderable {
    fn draw_text(
        &self,
        context: &DrawContext,
        text: impl AsRef<str>,
        position: impl Into<Point>,
        font: &Font,
    ) {
        self.draw_text_raw(context, text.as_ref(), position.into(), font);
    }
}

impl<T: ScreenRenderable + ?Sized> ScreenRenderableExt for T {}
