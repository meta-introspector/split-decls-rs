// Generated macro for impl_369 (impl)
macro_rules! Depcrate_stream_partialimpl_369 {
() => {
// Module: crate::stream::partial
// Provides: {"impl_369"}
// Dependencies: {}
impl < I > Partial < I > where I : StreamIsPartial , { # [doc = " Create a partial input"] # [inline] pub fn new (input : I) -> Self { debug_assert ! (! I :: is_partial_supported () , "`Partial` can only wrap complete sources") ; let partial = true ; Self { input , partial } } # [doc = " Extract the original [`Stream`]"] # [inline (always)] pub fn into_inner (self) -> I { self . input } }
};
}
