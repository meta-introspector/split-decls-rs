// Generated macro for rewrite_with_parens (function)
macro_rules! Depcrate_overflowrewrite_with_parens {
() => {
// Module: crate::overflow
// Provides: {"rewrite_with_parens"}
// Dependencies: {}
pub (crate) fn rewrite_with_parens < 'a , T : 'a + IntoOverflowableItem < 'a > > (context : & 'a RewriteContext < '_ > , ident : & 'a str , items : impl Iterator < Item = & 'a T > , shape : Shape , span : Span , item_max_width : usize , force_separator_tactic : Option < SeparatorTactic > ,) -> RewriteResult { Context :: new (context , items , ident , shape , span , "(" , ")" , item_max_width , force_separator_tactic , None ,) . rewrite (shape) }
};
}
