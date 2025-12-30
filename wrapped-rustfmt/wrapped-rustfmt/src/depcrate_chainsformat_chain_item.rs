// Generated macro for format_chain_item (function)
macro_rules! Depcrate_chainsformat_chain_item {
() => {
// Module: crate::chains
// Provides: {"format_chain_item"}
// Dependencies: {}
fn format_chain_item (item : & ChainItem , context : & RewriteContext < '_ > , rewrite_shape : Shape , allow_overflow : bool ,) -> RewriteResult { if allow_overflow { item . rewrite_result (context , rewrite_shape) . or_else (| _ | format_overflow_style (item . span , context) . unknown_error ()) } else { item . rewrite_result (context , rewrite_shape) } }
};
}
