use crate::script;
use crate::script::context::RuneheartError::{
    EmptyScript, RuneAllocError, RuneBuildError, RuneContextError, RuneDiagnosticError,
    RuneEmitError, RunePathError,
};
use crate::script::context::RuneheartExecutionError::{NoActiveScript, RuneVmError};
use crate::script::rune_module::JNIBlockContext;
use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jlong;
use rune::diagnostics::EmitError;
use rune::runtime::{RuntimeContext, VmError};
use rune::source::FromPathError;
use rune::termcolor::Buffer;
use rune::{BuildError, Context, ContextError, Diagnostics, Source, Sources, Unit, Value, Vm};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Debug)]
pub enum RuneheartError {
    EmptyScript,
    RuneContextError(ContextError),
    RuneAllocError(rune::alloc::Error),
    RuneBuildError(BuildError),
    RuneEmitError(EmitError),
    RuneDiagnosticError(String),
    RunePathError(FromPathError),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum RuneheartExecutionError {
    NoActiveScript,
    RuneVmError(VmError),
}

pub type RuneheartResult<T> = Result<T, RuneheartError>;

pub type RuneheartExecutionResult<T> = Result<T, RuneheartExecutionError>;

// TODO: our operations shoudl happen on this object not the overall context ,
pub struct ActiveScript {
    unit: Arc<Unit>,
    vm: Vm,
}

pub struct RuneheartContext {
    directory: Option<String>,
    tick_hash: rune::Hash,
    context: Context,
    runtime: Arc<RuntimeContext>,
    active_script: Option<ActiveScript>,
}

impl RuneheartContext {
    pub fn from_handle(handle: jlong) -> &'static Self {
        unsafe { &*(handle as usize as *mut RuneheartContext) }
    }

    pub fn from_handle_mut(handle: jlong) -> &'static mut Self {
        unsafe { &mut *(handle as usize as *mut RuneheartContext) }
    }

    fn compile_unit(&self, path: &Path) -> RuneheartResult<Unit> {
        let mut sources = Sources::new();
        sources
            .insert(Source::from_path(path).map_err(RunePathError)?)
            .map_err(RuneAllocError)?;

        let mut diagnostics = Diagnostics::new();

        let unit = rune::prepare(&mut sources)
            .with_diagnostics(&mut diagnostics)
            .with_context(&self.context)
            .build();

        if !diagnostics.is_empty() && diagnostics.has_error() {
            // TODO: https://docs.advntr.dev/serializer/ansi.html look into using this on kotlinm side
            let mut writer = Buffer::no_color();
            diagnostics
                .emit(&mut writer, &sources)
                .map_err(RuneEmitError)?;

            let diagnostic_data =
                String::from_utf8(writer.into_inner()).expect("invalid utf8 from diagnostics?");

            return Err(RuneDiagnosticError(diagnostic_data));
        }

        Ok(unit.map_err(RuneBuildError)?)
    }

    pub fn set_active_script(&mut self, path: &Path) -> RuneheartResult<()> {
        let unit = Arc::new(self.compile_unit(path)?);
        let vm = Vm::new(self.runtime.clone(), unit.clone());

        self.active_script = Some(ActiveScript { unit, vm });

        Ok(())
    }

    pub fn new() -> RuneheartResult<Self> {
        let mut context = Context::with_default_modules().map_err(RuneContextError)?;
        context.install(script::rune_module::module(true).map_err(RuneContextError)?).map_err(RuneContextError)?;
        let runtime = Arc::new(context.runtime().map_err(RuneAllocError)?);

        Ok(Self {
            directory: None,
            context,
            runtime,
            tick_hash: rune::Hash::type_hash(["tick"]),
            active_script: None,
        })
    }

    pub fn callback_tick(
        &mut self,
        jni_context: JNIBlockContext,
    ) -> RuneheartExecutionResult<Value> {
        match &mut self.active_script {
            None => Ok(Value::empty()),
            Some(script) => Ok(script
                .vm
                .execute(self.tick_hash, (jni_context,))
                .map_err(RuneVmError)?
                .complete()
                .into_result()
                .map_err(RuneVmError)?),
        }
    }

    #[cfg(test)]
    pub fn callback_tick_test(&mut self) -> RuneheartExecutionResult<Value> {
        match &mut self.active_script {
            None => Ok(Value::empty()),
            Some(script) => Ok(script
                .vm
                .execute(self.tick_hash, ())
                .map_err(RuneVmError)?
                .complete()
                .into_result()
                .map_err(RuneVmError)?),
        }
    }
}

// struct Block;
//
// fn move_from_to(Block, );
