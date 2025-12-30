// Generated macro for impl_335 (impl)
macro_rules! Depcrate_stream_locatingimpl_335 {
() => {
// Module: crate::stream::locating
// Provides: {"impl_335"}
// Dependencies: {}
impl < I > LocatingSlice < I > where I : Clone + Offset , { # [doc = " Wrap another Stream with span tracking"] pub fn new (input : I) -> Self { let initial = input . clone () ; Self { initial , input } } # [inline] fn previous_token_end (& self) -> usize { self . input . offset_from (& self . initial) } # [inline] fn current_token_start (& self) -> usize { self . input . offset_from (& self . initial) } }
};
}
