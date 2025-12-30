// Generated macro for rewrite_call (function)
macro_rules! Depcrate_exprrewrite_call {
() => {
// Module: crate::expr
// Provides: {"rewrite_call"}
// Dependencies: {}
pub (crate) fn rewrite_call (context : & RewriteContext < '_ > , callee : & str , args : & [ptr :: P < ast :: Expr >] , span : Span , shape : Shape ,) -> RewriteResult { overflow :: rewrite_with_parens (context , callee , args . iter () , shape , span , context . config . fn_call_width () , choose_separator_tactic (context , span) ,) }
};
}
