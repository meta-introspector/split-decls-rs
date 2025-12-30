// Generated macro for enum_variant_names_inner (function)
macro_rules! Depcrate_macros_enum_variant_namesenum_variant_names_inner {
() => {
// Module: crate::macros::enum_variant_names
// Provides: {"enum_variant_names_inner"}
// Dependencies: {}
pub fn enum_variant_names_inner (ast : & DeriveInput) -> syn :: Result < TokenStream > { let name = & ast . ident ; let gen = & ast . generics ; let (impl_generics , ty_generics , where_clause) = gen . split_for_impl () ; let variants = match & ast . data { Data :: Enum (v) => & v . variants , _ => return Err (non_enum_error ()) , } ; let type_properties = ast . get_type_properties () ? ; let strum_module_path = type_properties . crate_module_path () ; let names = variants . iter () . map (| v | { let props = v . get_variant_properties () ? ; Ok (props . get_preferred_name (type_properties . case_style , type_properties . prefix . as_ref () , type_properties . suffix . as_ref () ,)) }) . collect :: < syn :: Result < Vec < LitStr > > > () ? ; Ok (quote ! { impl # impl_generics # strum_module_path :: VariantNames for # name # ty_generics # where_clause { const VARIANTS : &'static [&'static str] = & [# (# names) ,*] ; } }) }
};
}
