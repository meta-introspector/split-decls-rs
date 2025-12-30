// Generated macro for rewrite_with_square_brackets (function)
macro_rules! Depcrate_overflowrewrite_with_square_brackets {
() => {
// Module: crate::overflow
// Provides: {"rewrite_with_square_brackets"}
// Dependencies: {}
pub (crate) fn rewrite_with_square_brackets < 'a , T : 'a + IntoOverflowableItem < 'a > > (context : & 'a RewriteContext < '_ > , name : & 'a str , items : impl Iterator < Item = & 'a T > , shape : Shape , span : Span , force_separator_tactic : Option < SeparatorTactic > , delim_token : Option < Delimiter > ,) -> RewriteResult { let (lhs , rhs) = match delim_token { Some (Delimiter :: Parenthesis) => ("(" , ")") , Some (Delimiter :: Brace) => ("{" , "}") , _ => ("[" , "]") , } ; Context :: new (context , items , name , shape , span , lhs , rhs , context . config . array_width () , force_separator_tactic , Some (("[" , "]")) ,) . rewrite (shape) }
};
}
