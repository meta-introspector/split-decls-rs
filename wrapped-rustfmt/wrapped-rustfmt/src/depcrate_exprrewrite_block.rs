// Generated macro for rewrite_block (function)
macro_rules! Depcrate_exprrewrite_block {
() => {
// Module: crate::expr
// Provides: {"rewrite_block"}
// Dependencies: {}
fn rewrite_block (block : & ast :: Block , attrs : Option < & [ast :: Attribute] > , label : Option < ast :: Label > , context : & RewriteContext < '_ > , shape : Shape ,) -> RewriteResult { rewrite_block_inner (block , attrs , label , true , context , shape) }
};
}
