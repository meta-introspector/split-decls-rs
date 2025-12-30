// Generated macro for impl_25 (impl)
macro_rules! Depcrate_spanimpl_25 {
() => {
// Module: crate::span
// Provides: {"impl_25"}
// Dependencies: {}
impl < T > Spanned < T > { # [doc = " Creates a new spanned object."] pub fn new (t : T , span : Span) -> Self { Self (t , span) } # [doc = " Returns a reference to the spanned node."] pub fn inner (& self) -> & T { & self . 0 } }
};
}
