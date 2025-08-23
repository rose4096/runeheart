mod context;

use crate::context::{RuneheartContext, RuneheartResult};
use jni::JNIEnv;
use jni::objects::{AutoLocal, JClass, JObject, JObjectArray, JString, JValue};
use jni::strings::JavaStr;
use jni::sys::{jlong, jobject, jstring};
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, Diagnostics, Hash, Source, Sources, Unit, Vm};
use std::collections::HashMap;
use std::iter::Map;
use std::panic;
use std::sync::{Arc, LazyLock, Mutex};

// #[unsafe(no_mangle)]
// pub extern "system" fn Java_rose_runeheart_Native_tick<'local>(
//     mut env: JNIEnv<'local>,
//     class: JClass<'local>,
//     relative_block_entities: JObject<'local>,
// ) {
//     let list = env
//         .get_list(&relative_block_entities)
//         .expect("not a java list");
//     let mut iter = list.iter(&mut env).expect("failed to create iterator");
//
//     while let Some(relative_block_entity) = iter.next(&mut env).expect("next not available") {
//         let relative_block_entity: AutoLocal<JObject> = env.auto_local(relative_block_entity);
//         let tostr = env
//             .call_method(
//                 &relative_block_entity,
//                 "toString",
//                 "()Ljava/lang/String;",
//                 &[],
//             )
//             .unwrap();
//         let obj = JString::from(tostr.l().unwrap());
//         let str = env.get_string(&obj).unwrap();
//         let rs = str.to_str().unwrap();
//         println!("{:#X?}", rs);
//     }
// }

#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_createContext<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    name: JString<'local>,
    script: JString<'local>,
) -> jlong {
    let name = env.get_string(&name);
    let script = env.get_string(&script);
    match (name, script) {
        (Ok(name), Ok(script)) => {
            let context = RuneheartContext::new(name.into(), script.into());
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

#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_deleteContext<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    context: jlong,
) {
    unsafe { drop(Box::from_raw(context as usize as *mut RuneheartContext)) };
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_tick<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    context: jlong,
) {
    let context = RuneheartContext::from_handle(context);
    println!("{:#X?}", context.name);
}


// #[unsafe(no_mangle)]
// pub extern "system" fn Java_rose_runeheart_Native_runScript<'local>(
//     mut env: JNIEnv<'local>,
//     class: JClass<'local>,
//     vm: jlong,
//     name: JString<'local>,
// ) {
//     // let mut vm: *mut Vm = unsafe { std::mem::transmute(vm) };
//     let vm: *mut Vm = vm as isize as *mut Vm;
//     unsafe {
//         println!("{:#X?}", (*vm).ip());
//     }
//
//     let output = unsafe {
//         (*vm)
//             .execute(["main"], (33i64,))
//             .unwrap()
//             .complete()
//             .into_result()
//             .unwrap()
//     };
//     let res: i64 = rune::from_value(output).unwrap();
//
//     println!("{:#X?}", res);
// }
