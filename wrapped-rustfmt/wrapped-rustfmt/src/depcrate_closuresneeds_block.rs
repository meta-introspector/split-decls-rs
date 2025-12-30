// Generated macro for needs_block (function)
macro_rules! Depcrate_closuresneeds_block {
() => {
// Module: crate::closures
// Provides: {"needs_block"}
// Dependencies: {}
fn needs_block (block : & ast :: Block , label : & Option < Label > , prefix : & str , context : & RewriteContext < '_ > ,) -> bool { let has_attributes = block . stmts . first () . map_or (false , | first_stmt | { ! get_attrs_from_stmt (first_stmt) . is_empty () }) ; is_unsafe_block (block) || block . stmts . len () > 1 || has_attributes || block_contains_comment (context , block) || prefix . contains ('\n') || label . is_some () }
};
}
