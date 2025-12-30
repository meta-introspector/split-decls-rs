// Generated macro for level_to_cs (function)
macro_rules! Depcratelevel_to_cs {
() => {
// Module: crate
// Provides: {"level_to_cs"}
// Dependencies: {}
fn level_to_cs (level : Level) -> (& 'static dyn Callsite , & 'static Fields) { match level { Level :: TRACE => (& TRACE_CS , & * TRACE_FIELDS) , Level :: DEBUG => (& DEBUG_CS , & * DEBUG_FIELDS) , Level :: INFO => (& INFO_CS , & * INFO_FIELDS) , Level :: WARN => (& WARN_CS , & * WARN_FIELDS) , Level :: ERROR => (& ERROR_CS , & * ERROR_FIELDS) , } }
};
}
