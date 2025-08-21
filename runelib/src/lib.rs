use jni::JNIEnv;
use jni::objects::{AutoLocal, JClass, JObject, JObjectArray, JString, JValue};
use jni::strings::JavaStr;
use jni::sys::{jobject, jstring};

#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_hello<'local>(
    mut env: JNIEnv<'local>,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    class: JClass<'local>,
    input: JString<'local>,
) -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let input: String = env
        .get_string(&input)
        .expect("Couldn't get java string!")
        .into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env
        .new_string(format!("hi from rust: input[{}]", input))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_raw()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_rose_runeheart_Native_tick<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    relative_block_entities: JObject<'local>,
) {
    let list = env.get_list(&relative_block_entities).expect("not a java list");
    let mut iter = list.iter(&mut env).expect("failed to create iterator");

    while let Some(relative_block_entity) = iter.next(&mut env).expect("next not available") {
        let relative_block_entity: AutoLocal<JObject> = env.auto_local(relative_block_entity);
        let tostr= env.call_method(&relative_block_entity, "toString", "()Ljava/lang/String;", &[]).unwrap();
        let obj = JString::from(tostr.l().unwrap());
        let str = env.get_string(&obj).unwrap();
        let rs = str.to_str().unwrap();
        println!("{:#X?}",rs);
    }
}
