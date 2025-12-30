// Generated macro for impl_345 (impl)
macro_rules! Depcrate_stream_locatingimpl_345 {
() => {
// Module: crate::stream::locating
// Provides: {"impl_345"}
// Dependencies: {}
impl < I > StreamIsPartial for LocatingSlice < I > where I : StreamIsPartial , { type PartialState = I :: PartialState ; # [inline] fn complete (& mut self) -> Self :: PartialState { self . input . complete () } # [inline] fn restore_partial (& mut self , state : Self :: PartialState) { self . input . restore_partial (state) ; } # [inline (always)] fn is_partial_supported () -> bool { I :: is_partial_supported () } # [inline (always)] fn is_partial (& self) -> bool { self . input . is_partial () } }
};
}
