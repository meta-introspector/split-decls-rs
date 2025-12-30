// Generated macro for allow_single_line_let_else_block (function)
macro_rules! Depcrate_itemsallow_single_line_let_else_block {
() => {
// Module: crate::items
// Provides: {"allow_single_line_let_else_block"}
// Dependencies: {}
fn allow_single_line_let_else_block (result : & str , block : & ast :: Block) -> bool { if result . contains ('\n') { return false ; } if block . stmts . len () <= 1 { return true ; } false }
};
}
