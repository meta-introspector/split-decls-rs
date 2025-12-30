// Generated macro for rewrite_chain (function)
macro_rules! Depcrate_chainsrewrite_chain {
() => {
// Module: crate::chains
// Provides: {"rewrite_chain"}
// Dependencies: {}
pub (crate) fn rewrite_chain (expr : & ast :: Expr , context : & RewriteContext < '_ > , shape : Shape ,) -> RewriteResult { let chain = Chain :: from_ast (expr , context) ; debug ! ("rewrite_chain {:?} {:?}" , chain , shape) ; if chain . children . is_empty () { return chain . parent . rewrite_result (context , shape) ; } chain . rewrite_result (context , shape) }
};
}
