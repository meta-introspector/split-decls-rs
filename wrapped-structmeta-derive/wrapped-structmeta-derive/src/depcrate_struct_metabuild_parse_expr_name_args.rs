// Generated macro for build_parse_expr_name_args (function)
macro_rules! Depcrate_struct_metabuild_parse_expr_name_args {
() => {
// Module: crate::struct_meta
// Provides: {"build_parse_expr_name_args"}
// Dependencies: {}
fn build_parse_expr_name_args (ty : & Type , is_vec : bool , span : Span) -> TokenStream { let value = if is_vec { quote_spanned ! (span => :: structmeta :: helpers :: exports :: syn :: punctuated :: Punctuated ::<# ty , :: structmeta :: helpers :: exports :: syn :: Token ! [,] >:: parse_terminated (& content) ?. into_iter () . collect ()) } else { quote_spanned ! (span => content . parse ::<# ty > () ?) } ; quote ! { { let content ; :: structmeta :: helpers :: exports :: syn :: parenthesized ! (content in input) ; # value } } }
};
}
