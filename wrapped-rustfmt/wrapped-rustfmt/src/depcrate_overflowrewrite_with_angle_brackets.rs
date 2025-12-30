// Generated macro for rewrite_with_angle_brackets (function)
macro_rules! Depcrate_overflowrewrite_with_angle_brackets {
() => {
// Module: crate::overflow
// Provides: {"rewrite_with_angle_brackets"}
// Dependencies: {}
pub (crate) fn rewrite_with_angle_brackets < 'a , T : 'a + IntoOverflowableItem < 'a > > (context : & 'a RewriteContext < '_ > , ident : & 'a str , items : impl Iterator < Item = & 'a T > , shape : Shape , span : Span ,) -> RewriteResult { Context :: new (context , items , ident , shape , span , "<" , ">" , context . config . max_width () , None , None ,) . rewrite (shape) }
};
}
