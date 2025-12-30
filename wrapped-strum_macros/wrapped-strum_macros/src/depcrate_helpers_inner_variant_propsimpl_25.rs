// Generated macro for impl_25 (impl)
macro_rules! Depcrate_helpers_inner_variant_propsimpl_25 {
() => {
// Module: crate::helpers::inner_variant_props
// Provides: {"impl_25"}
// Dependencies: {}
impl HasInnerVariantProperties for Field { fn get_variant_inner_properties (& self) -> syn :: Result < StrumInnerVariantProperties > { let mut output = StrumInnerVariantProperties { default_with : None } ; let mut default_with_kw = None ; for meta in self . get_named_metadata () ? { match meta { InnerVariantMeta :: DefaultWith { kw , value } => { if let Some (fst_kw) = default_with_kw { return Err (occurrence_error (fst_kw , kw , "default_with")) ; } default_with_kw = Some (kw) ; output . default_with = Some (value) ; } } } Ok (output) } }
};
}
