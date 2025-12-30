// Generated macro for impl_377 (impl)
macro_rules! Depcrate_stream_partialimpl_377 {
() => {
// Module: crate::stream::partial
// Provides: {"impl_377"}
// Dependencies: {}
impl < I > StreamIsPartial for Partial < I > where I : StreamIsPartial , { type PartialState = bool ; # [inline] fn complete (& mut self) -> Self :: PartialState { core :: mem :: replace (& mut self . partial , false) } # [inline] fn restore_partial (& mut self , state : Self :: PartialState) { self . partial = state ; } # [inline (always)] fn is_partial_supported () -> bool { true } # [inline (always)] fn is_partial (& self) -> bool { self . partial } }
};
}
