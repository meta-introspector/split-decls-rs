// Generated macro for non_unit_variant_error (function)
macro_rules! Depcrate_helpersnon_unit_variant_error {
() => {
// Module: crate::helpers
// Provides: {"non_unit_variant_error"}
// Dependencies: {}
pub fn non_unit_variant_error () -> syn :: Error { syn :: Error :: new (Span :: call_site () , "This macro only supports enums of strictly unit variants. Consider \
        using it in conjunction with [`EnumDiscriminants`]" ,) }
};
}
