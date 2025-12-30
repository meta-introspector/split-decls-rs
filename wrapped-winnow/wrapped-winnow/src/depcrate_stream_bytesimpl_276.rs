// Generated macro for impl_276 (impl)
macro_rules! Depcrate_stream_bytesimpl_276 {
() => {
// Module: crate::stream::bytes
// Provides: {"impl_276"}
// Dependencies: {}
impl StreamIsPartial for & Bytes { type PartialState = () ; # [inline] fn complete (& mut self) -> Self :: PartialState { } # [inline] fn restore_partial (& mut self , _state : Self :: PartialState) { } # [inline (always)] fn is_partial_supported () -> bool { false } }
};
}
