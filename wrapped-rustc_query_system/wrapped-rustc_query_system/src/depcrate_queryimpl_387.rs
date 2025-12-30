// Generated macro for impl_387 (impl)
macro_rules! Depcrate_queryimpl_387 {
() => {
// Module: crate::query
// Provides: {"impl_387"}
// Dependencies: {}
impl QueryStackFrameExtra { # [inline] pub fn new (description : String , span : Option < Span > , def_kind : Option < DefKind >) -> Self { Self { description , span , def_kind } } # [inline] pub fn default_span (& self , span : Span) -> Span { if ! span . is_dummy () { return span ; } self . span . unwrap_or (span) } }
};
}
