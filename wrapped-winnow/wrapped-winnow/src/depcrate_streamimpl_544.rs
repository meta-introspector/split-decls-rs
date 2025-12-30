// Generated macro for impl_544 (impl)
macro_rules! Depcrate_streamimpl_544 {
() => {
// Module: crate::stream
// Provides: {"impl_544"}
// Dependencies: {}
impl StreamIsPartial for & str { type PartialState = () ; # [inline] fn complete (& mut self) -> Self :: PartialState { } # [inline] fn restore_partial (& mut self , _state : Self :: PartialState) { } # [inline (always)] fn is_partial_supported () -> bool { false } }
};
}
