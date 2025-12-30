// Generated macro for impl_1304 (impl)
macro_rules! Depcrate_spannedimpl_1304 {
() => {
// Module: crate::spanned
// Provides: {"impl_1304"}
// Dependencies: {}
impl < T : Spanned > Spanned for Box < T > { fn span (& self) -> Span { (* * self) . span () } }
};
}
