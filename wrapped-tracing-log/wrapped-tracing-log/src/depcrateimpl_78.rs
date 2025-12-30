// Generated macro for impl_78 (impl)
macro_rules! Depcrateimpl_78 {
() => {
// Module: crate
// Provides: {"impl_78"}
// Dependencies: {}
impl AsTrace for log :: LevelFilter { type Trace = tracing_core :: LevelFilter ; # [inline] fn as_trace (& self) -> tracing_core :: LevelFilter { match self { log :: LevelFilter :: Off => tracing_core :: LevelFilter :: OFF , log :: LevelFilter :: Error => tracing_core :: LevelFilter :: ERROR , log :: LevelFilter :: Warn => tracing_core :: LevelFilter :: WARN , log :: LevelFilter :: Info => tracing_core :: LevelFilter :: INFO , log :: LevelFilter :: Debug => tracing_core :: LevelFilter :: DEBUG , log :: LevelFilter :: Trace => tracing_core :: LevelFilter :: TRACE , } } }
};
}
