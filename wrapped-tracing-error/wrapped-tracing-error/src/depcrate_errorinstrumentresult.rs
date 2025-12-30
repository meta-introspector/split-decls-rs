// Generated macro for InstrumentResult (trait)
macro_rules! Depcrate_errorInstrumentResult {
() => {
// Module: crate::error
// Provides: {"InstrumentResult"}
// Dependencies: {}
# [doc = " Extension trait for instrumenting errors in `Result`s with `SpanTrace`s"] # [cfg_attr (docsrs , doc (cfg (feature = "traced-error")))] pub trait InstrumentResult < T > { # [doc = " The type of the wrapped error after instrumentation"] type Instrumented ; # [doc = " Instrument an Error by bundling it with a SpanTrace"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use std::{io, fs};"] # [doc = " use tracing_error::{TracedError, InstrumentResult};"] # [doc = ""] # [doc = " # fn fallible_fn() -> io::Result<()> { fs::read_dir(\"......\").map(drop) };"] # [doc = ""] # [doc = " fn do_thing() -> Result<(), TracedError<io::Error>> {"] # [doc = "     fallible_fn().in_current_span()"] # [doc = " }"] # [doc = " ```"] fn in_current_span (self) -> Result < T , Self :: Instrumented > ; }
};
}
