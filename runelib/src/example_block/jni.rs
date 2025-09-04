use crate::example_block::screen::ExampleBlockScreen;
use crate::render::context::RenderContext;
use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::{jfloat, jint, jlong};

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

    if context.is_renderables_empty() {
        context.push_renderable(Box::new(ExampleBlockScreen::default()));
    }

    // multiply mouse_x/y by gui_scale so the position is accurate
    context.on_mouse_move(mouse_x * gui_scale as jint, mouse_y * gui_scale as jint);

    context.render_all();
}
