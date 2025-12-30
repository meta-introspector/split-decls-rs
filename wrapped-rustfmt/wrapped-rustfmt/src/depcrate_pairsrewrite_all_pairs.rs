// Generated macro for rewrite_all_pairs (function)
macro_rules! Depcrate_pairsrewrite_all_pairs {
() => {
// Module: crate::pairs
// Provides: {"rewrite_all_pairs"}
// Dependencies: {}
pub (crate) fn rewrite_all_pairs (expr : & ast :: Expr , shape : Shape , context : & RewriteContext < '_ > ,) -> RewriteResult { expr . flatten (context , shape) . unknown_error () . and_then (| list | { if list . let_chain_count () > 0 && ! list . can_rewrite_let_chain_single_line () { rewrite_pairs_multiline (& list , shape , context) } else { rewrite_pairs_one_line (& list , shape , context) . unknown_error () . or_else (| _ | rewrite_pairs_multiline (& list , shape , context)) } }) }
};
}
