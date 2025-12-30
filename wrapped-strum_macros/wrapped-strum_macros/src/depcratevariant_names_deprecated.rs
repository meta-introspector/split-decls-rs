// Generated macro for variant_names_deprecated (function)
macro_rules! Depcratevariant_names_deprecated {
() => {
// Module: crate
// Provides: {"variant_names_deprecated"}
// Dependencies: {}
# [doc (hidden)] # [proc_macro_derive (EnumVariantNames , attributes (strum))] # [deprecated (since = "0.26.0" , note = "please use `#[derive(VariantNames)]` instead")] pub fn variant_names_deprecated (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let ast = syn :: parse_macro_input ! (input as DeriveInput) ; let toks = macros :: enum_variant_names :: enum_variant_names_inner (& ast) . unwrap_or_else (| err | err . to_compile_error ()) ; debug_print_generated (& ast , & toks) ; toks . into () }
};
}
