// Generated macro for impl_545 (impl)
macro_rules! Depcrate_streamimpl_545 {
() => {
// Module: crate::stream
// Provides: {"impl_545"}
// Dependencies: {}
impl < I > StreamIsPartial for (I , usize) where I : StreamIsPartial , { type PartialState = I :: PartialState ; # [inline] fn complete (& mut self) -> Self :: PartialState { self . 0 . complete () } # [inline] fn restore_partial (& mut self , state : Self :: PartialState) { self . 0 . restore_partial (state) ; } # [inline (always)] fn is_partial_supported () -> bool { I :: is_partial_supported () } # [inline (always)] fn is_partial (& self) -> bool { self . 0 . is_partial () } }
};
}
