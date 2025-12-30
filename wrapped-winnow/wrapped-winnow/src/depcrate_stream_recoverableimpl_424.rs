// Generated macro for impl_424 (impl)
macro_rules! Depcrate_stream_recoverableimpl_424 {
() => {
// Module: crate::stream::recoverable
// Provides: {"impl_424"}
// Dependencies: {}
impl < I , E > StreamIsPartial for Recoverable < I , E > where I : StreamIsPartial , I : Stream , { type PartialState = I :: PartialState ; # [inline] fn complete (& mut self) -> Self :: PartialState { self . input . complete () } # [inline] fn restore_partial (& mut self , state : Self :: PartialState) { self . input . restore_partial (state) ; } # [inline (always)] fn is_partial_supported () -> bool { I :: is_partial_supported () } # [inline (always)] fn is_partial (& self) -> bool { self . input . is_partial () } }
};
}
