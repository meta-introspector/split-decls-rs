// Generated macro for rewrite_array (function)
macro_rules! Depcrate_exprrewrite_array {
() => {
// Module: crate::expr
// Provides: {"rewrite_array"}
// Dependencies: {}
pub (crate) fn rewrite_array < 'a , T : 'a + IntoOverflowableItem < 'a > > (name : & 'a str , exprs : impl Iterator < Item = & 'a T > , span : Span , context : & 'a RewriteContext < '_ > , shape : Shape , force_separator_tactic : Option < SeparatorTactic > , delim_token : Option < Delimiter > ,) -> RewriteResult { overflow :: rewrite_with_square_brackets (context , name , exprs , shape , span , force_separator_tactic , delim_token ,) }
};
}
