// Generated macro for rewrite_paren (function)
macro_rules! Depcrate_exprrewrite_paren {
() => {
// Module: crate::expr
// Provides: {"rewrite_paren"}
// Dependencies: {}
pub (crate) fn rewrite_paren (context : & RewriteContext < '_ > , mut subexpr : & ast :: Expr , shape : Shape , mut span : Span ,) -> RewriteResult { debug ! ("rewrite_paren, shape: {:?}" , shape) ; let mut pre_span ; let mut post_span ; let mut pre_comment ; let mut post_comment ; let remove_nested_parens = context . config . remove_nested_parens () ; loop { pre_span = mk_sp (span . lo () + BytePos (1) , subexpr . span () . lo ()) ; post_span = mk_sp (subexpr . span . hi () , span . hi () - BytePos (1)) ; pre_comment = rewrite_missing_comment (pre_span , shape , context) ? ; post_comment = rewrite_missing_comment (post_span , shape , context) ? ; if let ast :: ExprKind :: Paren (ref subsubexpr) = subexpr . kind { if remove_nested_parens && pre_comment . is_empty () && post_comment . is_empty () { span = subexpr . span ; subexpr = subsubexpr ; continue ; } } break ; } let sub_shape = shape . offset_left (1 , span) ? . sub_width (1 , span) ? ; let subexpr_str = subexpr . rewrite_result (context , sub_shape) ? ; let fits_single_line = ! pre_comment . contains ("//") && ! post_comment . contains ("//") ; if fits_single_line { Ok (format ! ("({pre_comment}{subexpr_str}{post_comment})")) } else { rewrite_paren_in_multi_line (context , subexpr , shape , pre_span , post_span) } }
};
}
