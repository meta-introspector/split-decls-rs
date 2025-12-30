// Generated macro for impl_543 (impl)
macro_rules! Depcrate_streamimpl_543 {
() => {
// Module: crate::stream
// Provides: {"impl_543"}
// Dependencies: {}
impl < T > StreamIsPartial for & [T] { type PartialState = () ; # [inline] fn complete (& mut self) -> Self :: PartialState { } # [inline] fn restore_partial (& mut self , _state : Self :: PartialState) { } # [inline (always)] fn is_partial_supported () -> bool { false } }
};
}
