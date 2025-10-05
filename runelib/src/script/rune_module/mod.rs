use jni::JNIEnv;
use jni::objects::{AsJArrayRaw, JByteArray, JObject, JObjectArray, JString, JValue, ReleaseMode};
use jni::signature::{JavaType, ReturnType};
use jni::sys::{jint, jobject, jsize};
use rune::alloc::clone::TryClone;
use rune::alloc::fmt::TryWrite;
use rune::runtime::{Args, Formatter, VmResult};
use rune::{Any, ContextError, Module, Value, vm_write};
use serde::{Deserialize, Serialize};
use std::ptr::NonNull;
use std::sync::Arc;

#[rune::module(::rune)]

pub fn module(_stdio: bool) -> Result<Module, ContextError> {
    let mut m = Module::from_meta(self::module_meta)?;
    m.ty::<Error>()?;
    m.function_meta(Error::display_fmt)?;
    m.function_meta(Error::debug_fmt)?;
    m.ty::<BlockEntityTarget>()?;

    m.ty::<Direction>()?;
    m.ty::<ScriptableItem>()?;
    m.ty::<ScriptableBlockEntity>()?;
    m.ty::<JNIBlockContext>()?;

    m.function_meta(JNIBlockContext::move_item)?;

    m.function_meta(ScriptableBlockEntity::display_fmt)?;
    m.function_meta(ScriptableBlockEntity::debug_fmt)?;

    m.ty::<BlockPos>()?;

    #[cfg(test)]
    m.function_meta(tests::get_block_entities_test)?;

    Ok(m)
}

#[derive(Any)]
#[rune(item = ::rune)]
#[derive(PartialEq, Debug, Deserialize, TryClone)]
pub struct BlockPos {
    #[rune(get)]
    pub x: i32,
    #[rune(get)]
    pub y: i32,
    #[rune(get)]
    pub z: i32,
}

#[derive(Any)]
#[rune(item = ::rune)]
#[derive(PartialEq, Debug, Deserialize, TryClone)]
pub struct ScriptableItem {
    slot_index: u32,
    #[rune(get)]
    pub name: String,
    #[rune(get)]
    pub tags: rune::alloc::Vec<String>,
    #[rune(get)]
    pub count: i32,
}

#[derive(Any)]
#[rune(item = ::rune)]
#[derive(PartialEq, Debug, Deserialize, TryClone)]
pub struct ScriptableBlockEntity {
    raw_access_index: u32,
    pub block_pos: BlockPos,
    #[rune(get)]
    pub dimension: String,
    #[rune(get)]
    pub name: String,
    #[rune(get)]
    pub items: rune::alloc::Vec<ScriptableItem>,
}

#[derive(Any)]
#[rune(item = ::rune)]
#[derive(PartialEq, Debug, TryClone)]
pub enum Direction {
    #[rune(constructor)]
    Down,
    #[rune(constructor)]
    Up,
    #[rune(constructor)]
    North,
    #[rune(constructor)]
    South,
    #[rune(constructor)]
    West,
    #[rune(constructor)]
    East,
}

impl Direction {
    fn to_str(&self) -> &str {
        match self {
            Direction::Down => "DOWN",
            Direction::Up => "UP",
            Direction::North => "NORTH",
            Direction::South => "SOUTH",
            Direction::West => "WEST",
            Direction::East => "EAST",
        }
    }
}

impl ScriptableBlockEntity {
    #[rune::function(protocol = DISPLAY_FMT)]
    pub fn display_fmt(&self, f: &mut Formatter) -> VmResult<()> {
        // todo lol
        self.__rune_fn__debug_fmt(f)
    }

    #[rune::function(protocol = DEBUG_FMT)]
    pub fn debug_fmt(&self, f: &mut Formatter) -> VmResult<()> {
        vm_write!(f, "{:#X?}", self)
    }
}

#[derive(Any)]
#[rune(item = ::rune)]
pub struct JNIBlockContext {
    raw_env: NonNull<jni::sys::JNIEnv>,
    block_entity: NonNull<jni::sys::_jobject>,
    raw_scriptable_entities: NonNull<jni::sys::_jobject>,
}

impl JNIBlockContext {
    pub fn new(
        env: &JNIEnv,
        block_entity: &JObject,
        raw_scriptable_entities: &JObjectArray,
    ) -> Self {
        Self {
            // unwrap: get_raw is assumed non-null
            raw_env: NonNull::new(env.get_raw()).unwrap(),
            block_entity: NonNull::new(block_entity.as_raw()).unwrap(),
            raw_scriptable_entities: NonNull::new(raw_scriptable_entities.as_raw()).unwrap(),
        }
    }

    fn env(&self) -> JNIEnv<'_> {
        // unsafe/unwrap: self.env is assumed non-null
        unsafe { JNIEnv::from_raw(self.raw_env.as_ptr()) }.unwrap()
    }

    fn block_entity(&self) -> JObject<'_> {
        unsafe { JObject::from_raw(self.block_entity.as_ptr()) }
    }

    fn raw_scriptable_entities(&self) -> JObjectArray {
        unsafe { JObjectArray::from_raw(self.raw_scriptable_entities.as_ptr()) }
    }

    fn get_raw_scriptable_entity(&self, index: u32) -> Option<JObject> {
        let mut env = self.env();
        let raw = self.raw_scriptable_entities();

        env.get_object_array_element(raw, index as jsize).ok()
    }

    #[rune::function]
    fn move_item(
        &self,
        src: &ScriptableBlockEntity,
        dst: &ScriptableBlockEntity,
        item: &ScriptableItem,
        face: Direction,
        amount: Option<i32>,
    ) -> Option<()> {
        let mut env = self.env();

        let src_raw = self.get_raw_scriptable_entity(src.raw_access_index)?;
        let dst_raw = self.get_raw_scriptable_entity(dst.raw_access_index)?;

        let dir_cls = env.find_class("net/minecraft/core/Direction").ok()?;
        let face = env
            .get_static_field(dir_cls, face.to_str(), "Lnet/minecraft/core/Direction;")
            .ok()?
            .l()
            .ok()?;

        let amount = if let Some(amount) = amount {
            env.new_object("java/lang/Integer", "(I)V", &[JValue::Int(amount as jint)])
                .ok()?
        } else {
            JObject::null()
        };

        env.call_method(
            self.block_entity(),
            "moveItem",
            "(Lrose/runeheart/blockentity/RawScriptableBlockEntity;Lrose/runeheart/blockentity/RawScriptableBlockEntity;ILnet/minecraft/core/Direction;Ljava/lang/Integer;)V",
            &[JValue::Object(&src_raw),JValue::Object(&dst_raw), JValue::Int(item.slot_index as jint), JValue::Object(&face), JValue::Object(&amount)],
        )
        .ok()?;

        Some(())
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
    use crate::script::context::{RuneheartContext, SourceKind};
    use std::path::Path;

    #[test]
    fn test_module() {
        let mut context = RuneheartContext::new().unwrap();
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("script")
            .join("rune_module")
            .join("test.rn");
        context.set_active_script(SourceKind::Path(path)).unwrap();

        let result = context.callback_tick_test().unwrap();
        let resultant = rune::from_value::<Result<u64, Error>>(result).unwrap();
        assert_eq!(resultant, Ok(123));
    }
}
