// Generated macro for block_has_statements (function)
macro_rules! Depcrate_exprblock_has_statements {
() => {
// Module: crate::expr
// Provides: {"block_has_statements"}
// Dependencies: {}
fn block_has_statements (block : & ast :: Block) -> bool { block . stmts . iter () . any (| stmt | ! matches ! (stmt . kind , ast :: StmtKind :: Empty)) }
};
}
