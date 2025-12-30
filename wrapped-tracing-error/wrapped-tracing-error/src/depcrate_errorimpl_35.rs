// Generated macro for impl_35 (impl)
macro_rules! Depcrate_errorimpl_35 {
() => {
// Module: crate::error
// Provides: {"impl_35"}
// Dependencies: {}
impl < E > InstrumentError for E where TracedError < E > : From < E > , { type Instrumented = TracedError < E > ; fn in_current_span (self) -> Self :: Instrumented { TracedError :: from (self) } }
};
}
