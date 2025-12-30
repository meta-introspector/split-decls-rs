// Generated macro for rewrite_block_inner (function)
macro_rules! Depcrate_exprrewrite_block_inner {
() => {
// Module: crate::expr
// Provides: {"rewrite_block_inner"}
// Dependencies: {}
fn rewrite_block_inner (block : & ast :: Block , attrs : Option < & [ast :: Attribute] > , label : Option < ast :: Label > , allow_single_line : bool , context : & RewriteContext < '_ > , shape : Shape ,) -> RewriteResult { let prefix = block_prefix (context , block , shape) ? ; if let Some (rw_str) = rewrite_empty_block (context , block , attrs , label , & prefix , shape) { return Ok (rw_str) ; } let result_str = rewrite_block_with_visitor (context , & prefix , block , attrs , label , shape , true) ? ; if allow_single_line && result_str . lines () . count () <= 3 { if let rw @ Ok (_) = rewrite_single_line_block (context , & prefix , block , attrs , label , shape) { return rw ; } } Ok (result_str) }
};
}
