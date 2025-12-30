// Generated macro for rewrite_unary_suffix (function)
macro_rules! Depcrate_exprrewrite_unary_suffix {
() => {
// Module: crate::expr
// Provides: {"rewrite_unary_suffix"}
// Dependencies: {}
pub (crate) fn rewrite_unary_suffix < R : Rewrite + Spanned > (context : & RewriteContext < '_ > , suffix : & str , rewrite : & R , shape : Shape ,) -> RewriteResult { let shape = shape . sub_width (suffix . len () , rewrite . span ()) ? ; rewrite . rewrite_result (context , shape) . map (| mut r | { r . push_str (suffix) ; r }) }
};
}
