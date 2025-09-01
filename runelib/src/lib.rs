mod context;
mod render_context;

use crate::context::RuneheartContext;
use crate::render_context::RenderContext;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jdouble, jint, jlong, jobject};
use skia_safe::textlayout::{ParagraphBuilder, ParagraphStyle, TextAlign, TextStyle};
use skia_safe::utils::text_utils::Align;
use skia_safe::{Color, Font, FontMgr, FontStyle, ISize, Paint, TextBlob, Typeface};

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_createRenderContext<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    width: jint,
    height: jint,
) -> jlong {
    Box::into_raw(Box::new(RenderContext::new(ISize::new(width, height)))) as jlong
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_deleteRenderContext<'local>(
    _: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
) {
    unsafe { drop(Box::from_raw(context as usize as *mut RenderContext)) };
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_getPixelBuffer<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
) -> jobject {
    let context = RenderContext::from_handle_mut(context);

    unsafe {
        env.new_direct_byte_buffer(context.buffer.as_mut_ptr(), context.buffer.len())
            .unwrap()
            .into_raw()
    }
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
    let context = RenderContext::from_handle_mut(context);
    context.resize_pixel_buffer(ISize::new(width, height));

    unsafe {
        env.new_direct_byte_buffer(context.buffer.as_mut_ptr(), context.buffer.len())
            .unwrap()
            .into_raw()
    }
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
    let context: &mut RenderContext = RenderContext::from_handle_mut(context);
    context.update_key_state(key_code, scan_mode, modifiers);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_onMousePressed<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
    button: jint,
) {
    let context: &mut RenderContext = RenderContext::from_handle_mut(context);
    context.update_mouse_press_state(button);
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
    let context: &mut RenderContext = RenderContext::from_handle_mut(context);
    context.update_mouse_scroll_state(delta_x, delta_y);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_render<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
    mouse_x: jint,
    mouse_y: jint,
) {
    let context: &mut RenderContext = RenderContext::from_handle_mut(context);

    context.draw(|context| {
        let key_state = &context.key_state;
        let mouse_button = &context.mouse_button;
        let scroll_delta = &context.mouse_scroll;
        let mono_ts = &context.mono_ts;
        // let regular_ts = &context.regular_ts;
        let size = context.size;

        if key_state.is_some() {
            println!("{:#X?}", key_state);
        }
        if mouse_button.is_some() {
            println!("{:#X?}", mouse_button);
        }
        if scroll_delta.is_some() {
            println!("{:#X?}", scroll_delta);
        }

        let (width, height) = (size.width, size.height);
        let center = (width / 2, height / 2);

        context.surface.canvas().clear(Color::from_argb(0, 0, 0, 0));
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(Color::BLACK);

        let paragraph_style = ParagraphStyle::new();
        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, &context.font_collection);
        let mut ts = TextStyle::new();
        ts.set_font_size(16.0);
        ts.set_foreground_paint(&Paint::default())
            .set_font_families(&["JetBrains Mono"]);
        paragraph_builder.push_style(&ts);
        paragraph_builder.add_text("JB MONO -> >= != ===");

        let mut paragraph = paragraph_builder.build();
        paragraph.layout(256.0);
        paragraph.paint(context.surface.canvas(), center);

        // let mut p_style = ParagraphStyle::new();
        // p_style.set_text_style(mono_ts);
        // let mut builder = ParagraphBuilder::new(&p_style, &context.font_collection);
        // builder.push_style(mono_ts);
        // builder.add_text("JB MONO -> >= != ===");
        // let mut para = builder.build();
        // para.layout(100.0);
        // para.paint(context.surface.canvas(), center);
    });
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_createContext<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    script: JString<'local>,
) -> jlong {
    let script = env.get_string(&script);
    match script {
        Ok(script) => {
            let context = RuneheartContext::new(script.into());
            match context {
                Ok(context) => {
                    let context = Box::new(context);
                    Box::into_raw(context) as jlong
                }
                Err(err) => {
                    env.throw_new("java/lang/RuntimeException", format!("{:?}", err))
                        .expect("failed to throw runtime exception?");
                    0
                }
            }
        }
        _ => 0,
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_deleteContext<'local>(
    _: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
) {
    unsafe { drop(Box::from_raw(context as usize as *mut RuneheartContext)) };
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_tick<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
) {
    let context = RuneheartContext::from_handle_mut(context);
    match context.callback_tick() {
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", format!("{:?}", err))
                .expect("failed to throw runtime exception?");
        }
        _ => {}
    }
}
