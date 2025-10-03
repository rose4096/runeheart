use crate::example_block::jni::ExampleBlockRenderData;
use crate::script::context::{RuneheartContext, RuneheartExecutionError};
use crate::script::context::RuneheartExecutionError::NoActiveScript;
use crate::script::rune_module::{JNIBlockContext, ScriptableBlockEntity};
use ciborium::from_reader;
use jni::JNIEnv;
use jni::objects::{JByteArray, JClass, JObject, JObjectArray, JString};
use jni::sys::{jlong, jobject};
use rune::runtime::VmError;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_createContext<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
) -> jlong {
    let context = RuneheartContext::new();
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
    object: JObject<'local>,
    raw: JObjectArray,
    scriptable: JByteArray<'local>,
) {
    // println!(
    //     "{:?}",
    //     from_reader::<Vec<ScriptableBlockEntity>, _>(
    //         &()[..]
    //     )
    // );

    if let Ok(bytes) = env.convert_byte_array(scriptable)
        && let Ok(scriptables) = from_reader::<Vec<ScriptableBlockEntity>, _>(&bytes[..])
    {
        let context = RuneheartContext::from_handle_mut(context);
        if let Err(err) =
            context.callback_tick(JNIBlockContext::new(&env, &object, &raw), scriptables)
        {
            println!("{:?}", err);
            // env.throw_new("java/lang/RuntimeException", format!("{:?}", err))
            //     .expect("failed to throw runtime exception?");
        }
    }
}
