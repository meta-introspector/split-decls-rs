// Generated macro for is_block_closure_forced (function)
macro_rules! Depcrate_closuresis_block_closure_forced {
() => {
// Module: crate::closures
// Provides: {"is_block_closure_forced"}
// Dependencies: {}
fn is_block_closure_forced (context : & RewriteContext < '_ > , expr : & ast :: Expr) -> bool { if context . inside_macro () { false } else { is_block_closure_forced_inner (expr , context . config . style_edition ()) } }
};
}
