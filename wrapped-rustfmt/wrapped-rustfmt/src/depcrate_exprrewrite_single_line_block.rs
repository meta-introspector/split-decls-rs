// Generated macro for rewrite_single_line_block (function)
macro_rules! Depcrate_exprrewrite_single_line_block {
() => {
// Module: crate::expr
// Provides: {"rewrite_single_line_block"}
// Dependencies: {}
fn rewrite_single_line_block (context : & RewriteContext < '_ > , prefix : & str , block : & ast :: Block , attrs : Option < & [ast :: Attribute] > , label : Option < ast :: Label > , shape : Shape ,) -> RewriteResult { if let Some (block_expr) = stmt :: Stmt :: from_simple_block (context , block , attrs) { let expr_shape = shape . offset_left (last_line_width (prefix) , block_expr . span ()) ? ; let expr_str = block_expr . rewrite_result (context , expr_shape) ? ; let label_str = rewrite_label (context , label) ; let result = format ! ("{prefix}{label_str}{{ {expr_str} }}") ; if result . len () <= shape . width && ! result . contains ('\n') { return Ok (result) ; } } Err (RewriteError :: Unknown) }
};
}
