use std::{fs, io};
use crate::example_block::screen::ExampleBlockScreen;
use crate::render::context::RenderContext;
use crate::script::context::{RuneheartContext, SourceKind};
use ciborium::{from_reader, into_writer};
use jni::JNIEnv;
use jni::objects::{JByteArray, JClass, JObject};
use jni::sys::{jbyteArray, jfloat, jint, jlong};
use serde::{Deserialize, Serialize};
use skia_safe::wrapper::NativeTransmutableWrapper;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct UIScript {
    pub file_name: String,
    pub full_path: PathBuf,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExampleBlockRenderData {
    pub scripts: Vec<UIScript>,
    pub target_directory: PathBuf,
    pub active_script: Option<UIScript>,
}

impl ExampleBlockRenderData {
    pub fn collect_directory(&mut self) -> io::Result<()> {
        self.scripts = fs::read_dir(&self.target_directory)?.filter_map(|file_path| {
            let file_path = file_path.ok()?;
            if file_path.path().extension()? == "rn" {
                return Some(UIScript{
                    file_name: file_path.file_name().to_string_lossy().to_string(),
                    full_path: file_path.path(),
                    content: fs::read_to_string(file_path.path()).ok()?,
                });
            }

            None
        }).collect();

        Ok(())
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_constructExampleBlockRenderData<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
) -> JByteArray<'local> {
    // TODO: clean up this horrible.
    let mut encoded: Vec<u8> = Vec::new();
    match into_writer::<ExampleBlockRenderData, _>(&ExampleBlockRenderData::default(), &mut encoded)
    {
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
pub extern "system" fn Java_rose_runeheart_Native_updateScriptContextFromRenderData<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    context: jlong,
    render_data_bytes: JByteArray<'local>,
) {
    if let Ok(bytes) = env.convert_byte_array(render_data_bytes)
        && let Ok(render_data) = from_reader::<ExampleBlockRenderData, _>(&bytes[..])
    {
        let context = RuneheartContext::from_handle_mut(context);
        if let Some(script) = render_data.active_script {
            context
                .set_active_script(SourceKind::Content(script.content.clone()))
                .unwrap();
        }
    }
}

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
    render_data_bytes: JByteArray<'local>,
) -> JObject<'local> {
    if let Ok(bytes) = env.convert_byte_array(render_data_bytes)
        && let Ok(render_data) = from_reader::<ExampleBlockRenderData, _>(&bytes[..])
    {
        let context: &mut RenderContext<ExampleBlockRenderData> =
            RenderContext::from_handle_mut(context);
        context.update_render_data(render_data);

        // multiply mouse_x/y by gui_scale so the position is accurate
        context.on_mouse_move(mouse_x * gui_scale as jint, mouse_y * gui_scale as jint);

        context.render_all();

        context
            .get_dirty_render_data(&mut env)
            .unwrap_or(JObject::null())
    } else {
        JObject::null()
    }
}
