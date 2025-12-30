// Generated macro for rewrite_tuple (function)
macro_rules! Depcrate_exprrewrite_tuple {
() => {
// Module: crate::expr
// Provides: {"rewrite_tuple"}
// Dependencies: {}
pub (crate) fn rewrite_tuple < 'a , T : 'a + IntoOverflowableItem < 'a > > (context : & 'a RewriteContext < '_ > , items : impl Iterator < Item = & 'a T > , span : Span , shape : Shape , is_singleton_tuple : bool ,) -> RewriteResult { debug ! ("rewrite_tuple {:?}" , shape) ; if context . use_block_indent () { let force_tactic = if context . inside_macro () { if span_ends_with_comma (context , span) { Some (SeparatorTactic :: Always) } else { Some (SeparatorTactic :: Never) } } else if is_singleton_tuple { Some (SeparatorTactic :: Always) } else { None } ; overflow :: rewrite_with_parens (context , "" , items , shape , span , context . config . fn_call_width () , force_tactic ,) } else { rewrite_tuple_in_visual_indent_style (context , items , span , shape , is_singleton_tuple) } }
};
}
