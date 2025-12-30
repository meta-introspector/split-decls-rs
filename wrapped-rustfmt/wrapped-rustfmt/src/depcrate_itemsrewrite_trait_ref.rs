// Generated macro for rewrite_trait_ref (function)
macro_rules! Depcrate_itemsrewrite_trait_ref {
() => {
// Module: crate::items
// Provides: {"rewrite_trait_ref"}
// Dependencies: {}
fn rewrite_trait_ref (context : & RewriteContext < '_ > , trait_ref : & ast :: TraitRef , offset : Indent , polarity_str : & str , result_len : usize ,) -> RewriteResult { let used_space = 1 + polarity_str . len () + result_len ; let shape = Shape :: indented (offset + used_space , context . config) ; if let Ok (trait_ref_str) = trait_ref . rewrite_result (context , shape) { if ! trait_ref_str . contains ('\n') { return Ok (format ! (" {polarity_str}{trait_ref_str}")) ; } } let offset = offset . block_indent (context . config) ; let shape = Shape :: indented (offset , context . config) ; let trait_ref_str = trait_ref . rewrite_result (context , shape) ? ; Ok (format ! ("{}{}{}" , offset . to_string_with_newline (context . config) , polarity_str , trait_ref_str)) }
};
}
