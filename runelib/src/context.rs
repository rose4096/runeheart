use crate::context::RuneheartError::{
    EmptyScript, InvalidName, RuneAllocError, RuneBuildError, RuneContextError,
    RuneDiagnosticError, RuneEmitError,
};
use jni::sys::jlong;
use rune::diagnostics::EmitError;
use rune::runtime::RuntimeContext;
use rune::termcolor::{Buffer, BufferWriter, ColorChoice, StandardStream};
use rune::{BuildError, Context, ContextError, Diagnostics, Source, Sources, Vm};
use std::sync::Arc;

#[derive(Debug)]
pub enum RuneheartError {
    InvalidName,
    EmptyScript,
    RuneContextError(ContextError),
    RuneAllocError(rune::alloc::Error),
    RuneBuildError(BuildError),
    RuneEmitError(EmitError),
    RuneDiagnosticError(String),
}

pub type RuneheartResult<T> = Result<T, RuneheartError>;

pub struct RuneheartContext {
    pub name: String,
    diagnostics: Diagnostics,
    vm: Vm,
}

impl RuneheartContext {
    pub fn from_handle(handle: jlong) -> &'static Self {
        unsafe { &*(handle as usize as *mut RuneheartContext) }
    }

    pub fn new(name: String, script: String) -> RuneheartResult<Self> {
        if name.is_empty() {
            return Err(InvalidName);
        }
        if script.is_empty() {
            return Err(EmptyScript);
        }

        let context = Context::with_default_modules().map_err(RuneContextError)?;
        let runtime = Arc::new(context.runtime().map_err(RuneAllocError)?);

        let mut sources = Sources::new();
        let mut diagnostics = Diagnostics::new();

        sources
            .insert(Source::memory(script).map_err(RuneAllocError)?)
            .map_err(RuneAllocError)?;

        let unit = rune::prepare(&mut sources)
            .with_context(&context)
            .with_diagnostics(&mut diagnostics)
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

        let unit = unit.map_err(RuneBuildError)?;
        let unit = Arc::new(unit);
        let vm = Vm::new(runtime, unit);

        Ok(Self {
            name,
            diagnostics,
            vm,
        })
    }
}
