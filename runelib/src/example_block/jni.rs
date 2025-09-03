use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::{jfloat, jint, jlong};
use skia_safe::{Color, Paint};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use crate::example_block::screen::ExampleBlockScreen;
use crate::render::context::RenderContext;
use crate::screen::ScreenRenderable;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_renderExampleBlock<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
    mouse_x: jint,
    mouse_y: jint,
    // TODO: find some way to respect gui scale maybe?
    // TODO: we currently just undo the gui scaling on the kotlin side... NOT GREAT!
    gui_scale: jfloat,
) {
    let context: &mut RenderContext = RenderContext::from_handle_mut(context);
    context.on_mouse_move(mouse_x, mouse_y);

    let example_block_screen = ExampleBlockScreen::default();

    context.with_canvas(|canvas, input, size, font_collection| {
        example_block_screen.render(canvas, input, size, font_collection);
    });

    // let key_state = &context.key_state;
    // let mouse_button = &context.mouse_button;
    // let scroll_delta = &context.mouse_scroll;
    // let size = context.size;
    //
    // if key_state.is_some() {
    //     println!("{:#X?}", key_state);
    // }
    // if mouse_button.is_some() {
    //     println!("{:#X?}", mouse_button);
    // }
    // if scroll_delta.is_some() {
    //     println!("{:#X?}", scroll_delta);
    // }
    //
    // let (width, height) = (size.width, size.height);
    // let center = (width / 2, height / 2);
    //
    // context.surface.canvas().clear(Color::from_argb(0, 0, 0, 0));
    // let mut paint = Paint::default();
    // paint.set_anti_alias(true);
    // paint.set_color(Color::BLACK);
    //
    // let paragraph_style = ParagraphStyle::new();
    // let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, &context.font_collection);
    // let mut ts = TextStyle::new();
    // let mut paint = Paint::default();
    // paint.set_anti_alias(true);
    // ts.set_font_size(16.0);
    // ts.set_foreground_paint(&paint)
    //     .set_font_families(&["JetBrains Mono"]);
    // paragraph_builder.push_style(&ts);
    // paragraph_builder.add_text("JB MONO -> >= != ===");
    //
    // let mut paragraph = paragraph_builder.build();
    // paragraph.layout(256.0);
    // paragraph.paint(context.surface.canvas(), center);

    context.end_draw();
}
