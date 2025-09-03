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
    _: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
    mouse_x: jint,
    mouse_y: jint,
    // TODO: find some way to respect gui scale maybe?
    // TODO: we currently just undo the gui scaling on the kotlin side... NOT GREAT!
    gui_scale: jfloat,
) {
    let context: &mut RenderContext = RenderContext::from_handle_mut(context);

    // multiply mouse_x/y by gui_scale so the position is accurate
    context.on_mouse_move(mouse_x * gui_scale as jint, mouse_y * gui_scale as jint);

    let example_block_screen = ExampleBlockScreen::default();

    context.with_canvas(|canvas, input, size, font_collection| {
        example_block_screen.render(canvas, input, size, font_collection);
    });

    context.end_draw();
}
