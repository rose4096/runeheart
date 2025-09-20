use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jlong, jobject};
use crate::script::context::RuneheartContext;
use crate::script::rune_module::JNIBlockContext;

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
    object: JObject<'local>,
) {
    let context = RuneheartContext::from_handle_mut(context);
    if let Err(err) = context.callback_tick(JNIBlockContext::new(&env, &object)) {
        env.throw_new("java/lang/RuntimeException", format!("{:?}", err))
            .expect("failed to throw runtime exception?");
    }
}
