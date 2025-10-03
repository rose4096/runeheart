use crate::example_block::jni::ExampleBlockRenderData;
use crate::example_block::screen::ExampleBlockScreen;
use crate::render::context::RenderContext;
use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::{jchar, jdouble, jint, jlong, jobject};
use skia_safe::ISize;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_createRenderContext<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    width: jint,
    height: jint,
) -> jlong {
    Box::into_raw(Box::new(RenderContext::<ExampleBlockRenderData>::new(
        ISize::new(width, height),
        Box::new(ExampleBlockScreen::new()),
    ))) as jlong
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_deleteRenderContext<'local>(
    _: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
) {
    unsafe {
        drop(Box::from_raw(
            context as usize as *mut RenderContext<ExampleBlockRenderData>,
        ))
    };
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_getPixelBuffer<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
) -> jobject {
    let context: &mut RenderContext<ExampleBlockRenderData> =
        RenderContext::from_handle_mut(context);

    context.create_byte_buffer(&mut env).unwrap().into_raw()
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_resizePixelBuffer<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
    width: jint,
    height: jint,
) -> jobject {
    let context: &mut RenderContext<ExampleBlockRenderData> =
        RenderContext::from_handle_mut(context);
    context.resize_pixel_buffer(ISize::new(width, height));

    context.create_byte_buffer(&mut env).unwrap().into_raw()
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_onKeyPressed<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
    key_code: jint,
    scan_mode: jint,
    modifiers: jint,
) {
    let context: &mut RenderContext<ExampleBlockRenderData> =
        RenderContext::from_handle_mut(context);
    context.on_key_pressed(key_code, scan_mode, modifiers);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_onKeyReleased<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
    key_code: jint,
    scan_mode: jint,
    modifiers: jint,
) {
    let context: &mut RenderContext<ExampleBlockRenderData> =
        RenderContext::from_handle_mut(context);
    context.on_key_released(key_code, scan_mode, modifiers);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_onMousePressed<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
    button: jint,
) {
    let context: &mut RenderContext<ExampleBlockRenderData> =
        RenderContext::from_handle_mut(context);
    context.on_mouse_pressed(button);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_onMouseReleased<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
) {
    let context: &mut RenderContext<ExampleBlockRenderData> =
        RenderContext::from_handle_mut(context);
    context.on_mouse_released();
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_onMouseScrolled<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
    delta_x: jdouble,
    delta_y: jdouble,
) {
    let context: &mut RenderContext<ExampleBlockRenderData> =
        RenderContext::from_handle_mut(context);
    context.on_mouse_scrolled(delta_x, delta_y);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_onCharacterTyped<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
    code_point: jchar,
    modifiers: jint,
) {
    let context: &mut RenderContext<ExampleBlockRenderData> =
        RenderContext::from_handle_mut(context);
    context.on_character_typed(code_point, modifiers);
}
