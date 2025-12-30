// Generated macro for impl_76 (impl)
macro_rules! Depcrateimpl_76 {
() => {
// Module: crate
// Provides: {"impl_76"}
// Dependencies: {}
impl AsTrace for log :: Level { type Trace = tracing_core :: Level ; # [inline] fn as_trace (& self) -> tracing_core :: Level { match self { log :: Level :: Error => tracing_core :: Level :: ERROR , log :: Level :: Warn => tracing_core :: Level :: WARN , log :: Level :: Info => tracing_core :: Level :: INFO , log :: Level :: Debug => tracing_core :: Level :: DEBUG , log :: Level :: Trace => tracing_core :: Level :: TRACE , } } }
};
}
