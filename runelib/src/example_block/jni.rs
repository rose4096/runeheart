use crate::example_block::screen::ExampleBlockScreen;
use crate::render::context::RenderContext;
use crate::script::context::RuneheartContext;
use ciborium::{from_reader, into_writer};
use jni::JNIEnv;
use jni::objects::{JByteArray, JClass};
use jni::sys::{jbyteArray, jfloat, jint, jlong};
use serde::{Deserialize, Serialize};
use skia_safe::wrapper::NativeTransmutableWrapper;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExampleBlockRenderData {
    scripts: Vec<(String, String)>,
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_getExampleBlockRenderData<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
) -> JByteArray<'local> {
    let context = RuneheartContext::from_handle(context);

    let data = ExampleBlockRenderData {
        scripts: vec![("test.rn".into(), "test".into())],
    };

    let mut encoded: Vec<u8> = Vec::new();
    match into_writer::<ExampleBlockRenderData, _>(&data, &mut encoded) {
        Ok(_) => match env.byte_array_from_slice(&encoded) {
            Ok(ok) => ok,
            Err(err) => {
                env.throw_new("java/lang/RuntimeException", format!("{:?}", err))
                    .expect("failed to throw runtime exception?");
                panic!("lol")
            }
        },
        Err(err) => {
            env.throw_new("java/lang/RuntimeException", format!("{:?}", err))
                .expect("failed to throw runtime exception?");
            panic!("lol")
        }
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_renderExampleBlock<'local>(
    env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
    mouse_x: jint,
    mouse_y: jint,
    // TODO: find some way to respect gui scale maybe?
    // TODO: we currently just undo the gui scaling on the kotlin side... NOT GREAT!
    gui_scale: jfloat,
    render_data_bytes: JByteArray<'local>,
) {
    if let Ok(bytes) = env.convert_byte_array(render_data_bytes)
        && let Ok(render_data) = from_reader::<ExampleBlockRenderData, _>(&bytes[..])
    {
        let context = RenderContext::from_handle_mut(context);
        if context.is_renderables_empty() {
            context.push_renderable(Box::new(ExampleBlockScreen::default()));
        }

        // multiply mouse_x/y by gui_scale so the position is accurate
        context.on_mouse_move(mouse_x * gui_scale as jint, mouse_y * gui_scale as jint);

        context.render_all(&render_data);
    }
}
