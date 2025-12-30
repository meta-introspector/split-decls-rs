// Generated macro for is_simple_block (function)
macro_rules! Depcrate_expris_simple_block {
() => {
// Module: crate::expr
// Provides: {"is_simple_block"}
// Dependencies: {}
pub (crate) fn is_simple_block (context : & RewriteContext < '_ > , block : & ast :: Block , attrs : Option < & [ast :: Attribute] > ,) -> bool { block . stmts . len () == 1 && stmt_is_expr (& block . stmts [0]) && ! block_contains_comment (context , block) && attrs . map_or (true , | a | a . is_empty ()) }
};
}
