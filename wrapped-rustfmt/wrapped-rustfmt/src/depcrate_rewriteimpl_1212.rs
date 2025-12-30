// Generated macro for impl_1212 (impl)
macro_rules! Depcrate_rewriteimpl_1212 {
() => {
// Module: crate::rewrite
// Provides: {"impl_1212"}
// Dependencies: {}
impl < 'a > RewriteContext < 'a > { pub (crate) fn snippet (& self , span : Span) -> & str { self . snippet_provider . span_to_snippet (span) . unwrap () } # [doc = " Returns `true` if we should use block indent style for rewriting function call."] pub (crate) fn use_block_indent (& self) -> bool { self . config . indent_style () == IndentStyle :: Block || self . use_block . get () } pub (crate) fn budget (& self , used_width : usize) -> usize { self . config . max_width () . saturating_sub (used_width) } pub (crate) fn inside_macro (& self) -> bool { self . inside_macro . get () } pub (crate) fn enter_macro (& self) -> InsideMacroGuard { let is_nested_macro_context = self . inside_macro . replace (true) ; InsideMacroGuard { is_nested_macro_context , inside_macro_ref : self . inside_macro . clone () , } } pub (crate) fn leave_macro (& self) { self . inside_macro . replace (false) ; } pub (crate) fn is_if_else_block (& self) -> bool { self . is_if_else_block . get () } }
};
}
