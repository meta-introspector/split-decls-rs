// Generated macro for non_single_field_variant_error (function)
macro_rules! Depcrate_helpersnon_single_field_variant_error {
() => {
// Module: crate::helpers
// Provides: {"non_single_field_variant_error"}
// Dependencies: {}
pub fn non_single_field_variant_error (attr : & str) -> syn :: Error { syn :: Error :: new (Span :: call_site () , format_args ! ("The [`{}`] attribute only supports enum variants with a single field" , attr) ,) }
};
}
