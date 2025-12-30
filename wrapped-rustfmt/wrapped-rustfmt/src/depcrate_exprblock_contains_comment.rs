// Generated macro for block_contains_comment (function)
macro_rules! Depcrate_exprblock_contains_comment {
() => {
// Module: crate::expr
// Provides: {"block_contains_comment"}
// Dependencies: {}
pub (crate) fn block_contains_comment (context : & RewriteContext < '_ > , block : & ast :: Block) -> bool { contains_comment (context . snippet (block . span)) }
};
}
