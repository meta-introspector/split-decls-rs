// Generated macro for format_header (function)
macro_rules! Depcrate_itemsformat_header {
() => {
// Module: crate::items
// Provides: {"format_header"}
// Dependencies: {}
fn format_header (context : & RewriteContext < '_ > , item_name : & str , ident : symbol :: Ident , vis : & ast :: Visibility , offset : Indent ,) -> String { let mut result = String :: with_capacity (128) ; let shape = Shape :: indented (offset , context . config) ; result . push_str (format_visibility (context , vis) . trim ()) ; let after_vis = vis . span . hi () ; if let Some (before_item_name) = context . snippet_provider . opt_span_before (mk_sp (vis . span . lo () , ident . span . hi ()) , item_name . trim ()) { let missing_span = mk_sp (after_vis , before_item_name) ; if let Ok (result_with_comment) = combine_strs_with_missing_comments (context , & result , item_name , missing_span , shape , true ,) { result = result_with_comment ; } } result . push_str (rewrite_ident (context , ident)) ; result }
};
}
