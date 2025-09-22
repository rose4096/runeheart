use jni::JNIEnv;
use jni::objects::JObject;
use jni::sys::jobject;
use rune::alloc::fmt::TryWrite;
use rune::runtime::{Formatter, VmResult};
use rune::{Any, ContextError, Module, Value, vm_write};
use std::ptr::NonNull;
use std::sync::Arc;

#[rune::module(::rune)]

pub fn module(_stdio: bool) -> Result<Module, ContextError> {
    let mut m = Module::from_meta(self::module_meta)?;
    m.ty::<Error>()?;
    m.function_meta(Error::display_fmt)?;
    m.function_meta(Error::debug_fmt)?;
    m.ty::<BlockEntityTarget>()?;

    m.ty::<JNIBlockContext>()?;
    m.function_meta(JNIBlockContext::get_block_entities)?;

    #[cfg(test)]
    m.function_meta(tests::get_block_entities_test)?;

    Ok(m)
}

#[derive(Any)]
#[rune(item = ::rune)]
pub struct JNIBlockContext {
    raw: NonNull<jni::sys::JNIEnv>,
    block_entity: NonNull<jni::sys::_jobject>,
}

impl JNIBlockContext {
    pub fn new(env: &JNIEnv, block_entity: &JObject) -> Self {
        Self {
            // unwrap: get_raw is assumed non-null
            raw: NonNull::new(env.get_raw()).unwrap(),
            block_entity: NonNull::new(block_entity.as_raw()).unwrap(),
        }
    }

    fn env(&self) -> JNIEnv<'_> {
        // unsafe/unwrap: self.env is assumed non-null
        unsafe { JNIEnv::from_raw(self.raw.as_ptr()) }.unwrap()
    }

    fn block_entity(&self) -> JObject<'_> {
        unsafe { JObject::from_raw(self.block_entity.as_ptr()) }
    }

    #[rune::function]
    fn get_block_entities(&self, target: BlockEntityTarget) -> i32 {
        let mut env = self.env();
        let be = self.block_entity();

        let result = env.call_method(be, "test_get_data", "()I", &[]).unwrap();
        result.i().unwrap()
    }
}

#[derive(Any)]
#[rune(item = ::rune)]
#[derive(PartialEq, Debug)]
struct Error {
    message: String,
}

impl Error {
    #[rune::function(protocol = DISPLAY_FMT)]
    pub fn display_fmt(&self, f: &mut Formatter) -> VmResult<()> {
        vm_write!(f, "{}", self.message)
    }

    #[rune::function(protocol = DEBUG_FMT)]
    pub fn debug_fmt(&self, f: &mut Formatter) -> VmResult<()> {
        vm_write!(f, "{}", self.message)
    }
}

#[derive(Any)]
#[rune(item = ::rune)]
enum BlockEntityTarget {
    #[rune(constructor)]
    Single,
    #[rune(constructor)]
    Multi,
    #[rune(constructor)]
    All,
}

#[cfg(test)]
mod tests {
    #[rune::function]
    fn get_block_entities_test(target: BlockEntityTarget) -> Result<u64, Error> {
        Ok(123)
    }

    use super::*;
    use crate::script::context::RuneheartContext;
    use std::path::Path;

    #[test]
    fn test_module() {
        let mut context = RuneheartContext::new().unwrap();
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("script")
            .join("rune_module")
            .join("test.rn");
        context.set_active_script(&path).unwrap();

        let result = context.callback_tick_test().unwrap();
        let resultant = rune::from_value::<Result<u64, Error>>(result).unwrap();
        assert_eq!(resultant, Ok(123));
    }
}
