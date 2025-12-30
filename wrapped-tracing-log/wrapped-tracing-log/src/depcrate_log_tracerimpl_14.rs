// Generated macro for impl_14 (impl)
macro_rules! Depcrate_log_tracerimpl_14 {
() => {
// Module: crate::log_tracer
// Provides: {"impl_14"}
// Dependencies: {}
impl log :: Log for LogTracer { fn enabled (& self , metadata : & log :: Metadata < '_ >) -> bool { if metadata . level () . as_trace () > tracing_core :: LevelFilter :: current () { return false ; } if ! self . ignore_crates . is_empty () { let target = metadata . target () ; for ignored in & self . ignore_crates [..] { if target . starts_with (ignored) { return false ; } } } try_cache_interest (metadata , | | { dispatcher :: get_default (| dispatch | dispatch . enabled (& metadata . as_trace ())) }) } fn log (& self , record : & log :: Record < '_ >) { if self . enabled (record . metadata ()) { crate :: dispatch_record (record) ; } } fn flush (& self) { } }
};
}
