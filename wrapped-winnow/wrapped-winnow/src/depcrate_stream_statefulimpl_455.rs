// Generated macro for impl_455 (impl)
macro_rules! Depcrate_stream_statefulimpl_455 {
() => {
// Module: crate::stream::stateful
// Provides: {"impl_455"}
// Dependencies: {}
impl < I , S > StreamIsPartial for Stateful < I , S > where I : StreamIsPartial , { type PartialState = I :: PartialState ; # [inline] fn complete (& mut self) -> Self :: PartialState { self . input . complete () } # [inline] fn restore_partial (& mut self , state : Self :: PartialState) { self . input . restore_partial (state) ; } # [inline (always)] fn is_partial_supported () -> bool { I :: is_partial_supported () } # [inline (always)] fn is_partial (& self) -> bool { self . input . is_partial () } }
};
}
