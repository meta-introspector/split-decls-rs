// Generated macro for impl_20 (impl)
macro_rules! Depcrate_errorimpl_20 {
() => {
// Module: crate::error
// Provides: {"impl_20"}
// Dependencies: {}
impl < E > From < E > for TracedError < E > where E : Error + Send + Sync + 'static , { fn from (error : E) -> Self { let vtable = & ErrorVTable { object_ref : object_ref :: < E > , } ; Self { inner : ErrorImpl { vtable , span_trace : SpanTrace :: capture () , error , } , } } }
};
}
