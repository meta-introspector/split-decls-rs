// Generated macro for rewrite_unary_prefix (function)
macro_rules! Depcrate_exprrewrite_unary_prefix {
() => {
// Module: crate::expr
// Provides: {"rewrite_unary_prefix"}
// Dependencies: {}
pub (crate) fn rewrite_unary_prefix < R : Rewrite + Spanned > (context : & RewriteContext < '_ > , prefix : & str , rewrite : & R , shape : Shape ,) -> RewriteResult { let shape = shape . offset_left (prefix . len () , rewrite . span ()) ? ; rewrite . rewrite_result (context , shape) . map (| r | format ! ("{}{}" , prefix , r)) }
};
}
