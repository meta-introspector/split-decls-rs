// Generated macro for is_empty_block (function)
macro_rules! Depcrate_expris_empty_block {
() => {
// Module: crate::expr
// Provides: {"is_empty_block"}
// Dependencies: {}
# [doc = " Checks whether a block contains no statements, expressions, comments, or"] # [doc = " inner attributes."] pub (crate) fn is_empty_block (context : & RewriteContext < '_ > , block : & ast :: Block , attrs : Option < & [ast :: Attribute] > ,) -> bool { ! block_has_statements (block) && ! block_contains_comment (context , block) && attrs . map_or (true , | a | inner_attributes (a) . is_empty ()) }
};
}
