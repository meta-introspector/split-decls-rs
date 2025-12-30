// Generated macro for impl_1171 (impl)
macro_rules! Depcrate_patternsimpl_1171 {
() => {
// Module: crate::patterns
// Provides: {"impl_1171"}
// Dependencies: {}
impl Rewrite for PatField { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { let hi_pos = if let Some (last) = self . attrs . last () { last . span . hi () } else { self . pat . span . lo () } ; let attrs_str = if self . attrs . is_empty () { String :: from ("") } else { self . attrs . rewrite_result (context , shape) ? } ; let pat_str = self . pat . rewrite_result (context , shape) ? ; if self . is_shorthand { combine_strs_with_missing_comments (context , & attrs_str , & pat_str , mk_sp (hi_pos , self . pat . span . lo ()) , shape , false ,) } else { let nested_shape = shape . block_indent (context . config . tab_spaces ()) ; let id_str = rewrite_ident (context , self . ident) ; let one_line_width = id_str . len () + 2 + pat_str . len () ; let pat_and_id_str = if one_line_width <= shape . width { format ! ("{id_str}: {pat_str}") } else { format ! ("{}:\n{}{}" , id_str , nested_shape . indent . to_string (context . config) , self . pat . rewrite_result (context , nested_shape) ?) } ; combine_strs_with_missing_comments (context , & attrs_str , & pat_and_id_str , mk_sp (hi_pos , self . pat . span . lo ()) , nested_shape , false ,) } } }
};
}
