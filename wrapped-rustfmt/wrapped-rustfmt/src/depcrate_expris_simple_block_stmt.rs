// Generated macro for is_simple_block_stmt (function)
macro_rules! Depcrate_expris_simple_block_stmt {
() => {
// Module: crate::expr
// Provides: {"is_simple_block_stmt"}
// Dependencies: {}
# [doc = " Checks whether a block contains at most one statement or expression, and no"] # [doc = " comments or attributes."] pub (crate) fn is_simple_block_stmt (context : & RewriteContext < '_ > , block : & ast :: Block , attrs : Option < & [ast :: Attribute] > ,) -> bool { block . stmts . len () <= 1 && ! block_contains_comment (context , block) && attrs . map_or (true , | a | a . is_empty ()) }
};
}
