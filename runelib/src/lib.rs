mod context;
mod render_context;

use crate::context::RuneheartContext;
use crate::render_context::RenderContext;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint, jlong, jobject};
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
pub extern "system" fn Java_rose_runeheart_Native_render<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
) {
    let context: &mut RenderContext = RenderContext::from_handle_mut(context);

    let canvas = context.canvas();

    canvas.clear(Color::from_argb(255, 255, 255, 255));
    let mut paint = Paint::default();

    paint.set_anti_alias(true);
    paint.set_argb(255, 90, 200, 120);
    canvas.draw_circle((64, 64), 50.0, &paint);

    let font_mgr = FontMgr::new();
    let default_typeface = font_mgr
        .legacy_make_typeface(None, FontStyle::default())
        .unwrap();
    let default_font = Font::new(default_typeface, 30.0);

    canvas.draw_text_align(
        "aewfawefaw",
        (200, 200),
        &default_font,
        &paint,
        Align::Right,
    );
    let text = TextBlob::from_str("HELLO RUNEHEART", &default_font).unwrap();
    canvas.draw_text_blob(&text, (100, 25), &paint);

    context.fill_pixel_buffer();
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
