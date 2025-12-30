// Generated macro for InstrumentError (trait)
macro_rules! Depcrate_errorInstrumentError {
() => {
// Module: crate::error
// Provides: {"InstrumentError"}
// Dependencies: {}
# [doc = " Extension trait for instrumenting errors with `SpanTrace`s"] # [cfg_attr (docsrs , doc (cfg (feature = "traced-error")))] pub trait InstrumentError { # [doc = " The type of the wrapped error after instrumentation"] type Instrumented ; # [doc = " Instrument an Error by bundling it with a SpanTrace"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use tracing_error::{TracedError, InstrumentError};"] # [doc = ""] # [doc = " fn wrap_error<E>(e: E) -> TracedError<E>"] # [doc = " where"] # [doc = "     E: std::error::Error + Send + Sync + 'static"] # [doc = " {"] # [doc = "     e.in_current_span()"] # [doc = " }"] # [doc = " ```"] fn in_current_span (self) -> Self :: Instrumented ; }
};
}
