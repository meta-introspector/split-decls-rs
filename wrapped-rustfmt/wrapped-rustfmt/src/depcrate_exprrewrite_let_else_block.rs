// Generated macro for rewrite_let_else_block (function)
macro_rules! Depcrate_exprrewrite_let_else_block {
() => {
// Module: crate::expr
// Provides: {"rewrite_let_else_block"}
// Dependencies: {}
# [doc = " Rewrite the divergent block of a `let-else` statement."] pub (crate) fn rewrite_let_else_block (block : & ast :: Block , allow_single_line : bool , context : & RewriteContext < '_ > , shape : Shape ,) -> RewriteResult { rewrite_block_inner (block , None , None , allow_single_line , context , shape) }
};
}
