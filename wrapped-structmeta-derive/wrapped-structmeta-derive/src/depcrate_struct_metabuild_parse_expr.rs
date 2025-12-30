// Generated macro for build_parse_expr (function)
macro_rules! Depcrate_struct_metabuild_parse_expr {
() => {
// Module: crate::struct_meta
// Provides: {"build_parse_expr"}
// Dependencies: {}
fn build_parse_expr (ty : & Type , span : Span) -> TokenStream { quote_spanned ! (span => input . parse ::<# ty > () ?) }
};
}
