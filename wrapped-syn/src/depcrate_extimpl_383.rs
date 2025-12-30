// Generated macro for impl_383 (impl)
macro_rules! Depcrate_extimpl_383 {
() => {
// Module: crate::ext
// Provides: {"impl_383"}
// Dependencies: {}
impl PunctExt for Punct { fn new_spanned (ch : char , spacing : Spacing , span : Span) -> Self { let mut punct = Punct :: new (ch , spacing) ; punct . set_span (span) ; punct } }
};
}
