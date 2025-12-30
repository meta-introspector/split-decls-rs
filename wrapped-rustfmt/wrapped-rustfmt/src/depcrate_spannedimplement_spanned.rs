// Generated macro for implement_spanned (macro)
macro_rules! Depcrate_spannedimplement_spanned {
() => {
// Module: crate::spanned
// Provides: {"implement_spanned"}
// Dependencies: {}
macro_rules ! implement_spanned { ($ this : ty) => { impl Spanned for $ this { fn span (& self) -> Span { span_with_attrs ! (self) } } } ; }
};
}
