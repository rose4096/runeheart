pub mod text_input;

use crate::render::input::{Input, MouseButton};
use skia_safe::textlayout::{
    FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextStyle,
};
use skia_safe::wrapper::PointerWrapper;
use skia_safe::{Canvas, Color, FontStyle, ISize, Paint, Point, Rect, Size, scalar};
use std::any::Any;

#[derive(Debug)]
pub enum Font {
    Regular(scalar, Color),
    Mono(scalar, Color),
}

impl Font {
    pub fn get_font(&self, font_collection: &FontCollection) -> Option<skia_safe::Font> {
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
        Some(skia_safe::Font::from_typeface(tfs.first()?, *size))
    }

    pub fn measure_text(
        &self,
        text: &str,
        paint: Option<&Paint>,
        font_collection: &FontCollection,
    ) -> Option<(scalar, Rect)> {
        let font = self.get_font(font_collection)?;
        Some(font.measure_text(text, paint))
    }

    pub fn measure_text_bounds(
        &self,
        text: &str,
        paint: Option<&Paint>,
        font_collection: &FontCollection,
    ) -> Option<Rect> {
        let (_, rect) = self.measure_text(text, paint, font_collection)?;
        let sk_font = self.get_font(font_collection)?;
        let metrics = sk_font.metrics().1;
        Some(rect.with_offset((0.0, metrics.top.abs())))
    }

    pub fn measure_height(&self, font_collection: &FontCollection) -> Option<scalar> {
        let font = self.get_font(font_collection)?;
        let metrics = font.metrics();
        // maybe use ascent/descent
        Some(metrics.1.top.abs() + metrics.1.bottom.abs())
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

pub trait ScreenRenderable<T> {
    fn paragraph(
        &self,
        context: &DrawContext,
        text: &str,
        font: &Font,
        paragraph_style: Option<ParagraphStyle>,
    ) -> Paragraph {
        let mut paragraph_style = paragraph_style.unwrap_or_default();
        paragraph_style.set_replace_tab_characters(true);

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

        paragraph_builder.build()
    }

    fn draw_text_raw(&self, context: &DrawContext, paragraph: Paragraph, position: Point) {
        paragraph.paint(context.canvas, position);
    }

    fn render(
        &mut self,
        canvas: &Canvas,
        input: &Input,
        screen_size: &ISize,
        font_collection: &FontCollection,
        render_data: &mut T,
    );
}

pub trait ScreenRenderableExt<T>: ScreenRenderable<T> {
    fn draw_text(
        &self,
        context: &DrawContext,
        text: impl AsRef<str>,
        position: impl Into<Point>,
        font: &Font,
    ) {
        let point = position.into();
        let mut paragraph = self.paragraph(context, text.as_ref(), font, None);
        paragraph.layout(100000.0);
        self.draw_text_raw(context, paragraph, point);
    }

    fn draw_paragraph(&self, context: &DrawContext, paragraph: Paragraph, point: impl Into<Point>) {
        self.draw_text_raw(context, paragraph, point.into());
    }
}

impl<D, R> ScreenRenderableExt<D> for R where R: ScreenRenderable<D> + ?Sized {}
