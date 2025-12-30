// Generated macro for flatten_arm_body (function)
macro_rules! Depcrate_matchesflatten_arm_body {
() => {
// Module: crate::matches
// Provides: {"flatten_arm_body"}
// Dependencies: {}
fn flatten_arm_body < 'a > (context : & 'a RewriteContext < '_ > , body : & 'a ast :: Expr , opt_shape : Option < Shape > ,) -> (bool , & 'a ast :: Expr) { let can_extend = | expr | ! context . config . force_multiline_blocks () && can_flatten_block_around_this (expr) ; if let Some (block) = block_can_be_flattened (context , body) { if let ast :: StmtKind :: Expr (ref expr) = block . stmts [0] . kind { if let ast :: ExprKind :: Block (..) = expr . kind { if expr . attrs . is_empty () { flatten_arm_body (context , expr , None) } else { (true , body) } } else { let cond_becomes_multi_line = opt_shape . and_then (| shape | rewrite_cond (context , expr , shape)) . map_or (false , | cond | cond . contains ('\n')) ; if cond_becomes_multi_line { (false , & * body) } else { (can_extend (expr) , & * expr) } } } else { (false , & * body) } } else { (can_extend (body) , & * body) } }
};
}
