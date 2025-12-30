// Generated macro for rewrite_empty_macro_def_body (function)
macro_rules! Depcrate_macrosrewrite_empty_macro_def_body {
() => {
// Module: crate::macros
// Provides: {"rewrite_empty_macro_def_body"}
// Dependencies: {}
fn rewrite_empty_macro_def_body (context : & RewriteContext < '_ > , span : Span , shape : Shape ,) -> RewriteResult { let block = ast :: Block { stmts : vec ! [] . into () , id : rustc_ast :: node_id :: DUMMY_NODE_ID , rules : ast :: BlockCheckMode :: Default , span , tokens : None , } ; block . rewrite_result (context , shape) }
};
}
