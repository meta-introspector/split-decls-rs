// Generated macro for lit_ends_in_dot (function)
macro_rules! Depcrate_exprlit_ends_in_dot {
() => {
// Module: crate::expr
// Provides: {"lit_ends_in_dot"}
// Dependencies: {}
pub (crate) fn lit_ends_in_dot (lit : & Lit , context : & RewriteContext < '_ >) -> bool { match lit . kind { LitKind :: Float => float_lit_ends_in_dot (lit . symbol . as_str () , lit . suffix . as_ref () . map (| s | s . as_str ()) , context . config . float_literal_trailing_zero () ,) , _ => false , } }
};
}
