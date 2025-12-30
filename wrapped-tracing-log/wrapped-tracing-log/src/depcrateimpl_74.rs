// Generated macro for impl_74 (impl)
macro_rules! Depcrateimpl_74 {
() => {
// Module: crate
// Provides: {"impl_74"}
// Dependencies: {}
impl AsLog for tracing_core :: Level { type Log = log :: Level ; fn as_log (& self) -> log :: Level { match * self { tracing_core :: Level :: ERROR => log :: Level :: Error , tracing_core :: Level :: WARN => log :: Level :: Warn , tracing_core :: Level :: INFO => log :: Level :: Info , tracing_core :: Level :: DEBUG => log :: Level :: Debug , tracing_core :: Level :: TRACE => log :: Level :: Trace , } } }
};
}
