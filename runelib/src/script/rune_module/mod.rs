use rune::alloc::fmt::TryWrite;
use rune::{vm_write, Any, ContextError, Module, Value};
use rune::runtime::{Formatter, VmResult};

#[rune::module(::rune)]

pub fn module(_stdio: bool) -> Result<Module, ContextError> {
    let mut m = Module::from_meta(self::module_meta)?;
    m.ty::<Error>()?;
    m.function_meta(Error::display_fmt)?;
    m.function_meta(Error::debug_fmt)?;

    Ok(m)
}

#[derive(Any)]
#[rune(item = ::rune)]
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
enum BlockEntityTarget {
    Single,
    Multi,
    All
}

#[rune::function]
fn get_block_entities(target: BlockEntityTarget) -> Result<Value, Error> {
    Ok(().into())
}