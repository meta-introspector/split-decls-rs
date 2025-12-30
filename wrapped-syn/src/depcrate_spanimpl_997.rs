// Generated macro for impl_997 (impl)
macro_rules! Depcrate_spanimpl_997 {
() => {
// Module: crate::span
// Provides: {"impl_997"}
// Dependencies: {}
impl IntoSpans < DelimSpan > for Span { fn into_spans (self) -> DelimSpan { let mut group = Group :: new (Delimiter :: None , TokenStream :: new ()) ; group . set_span (self) ; group . delim_span () } }
};
}
