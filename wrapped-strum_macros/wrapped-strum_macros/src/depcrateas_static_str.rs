// Generated macro for as_static_str (function)
macro_rules! Depcrateas_static_str {
() => {
// Module: crate
// Provides: {"as_static_str"}
// Dependencies: {}
# [proc_macro_derive (AsStaticStr , attributes (strum))] # [doc (hidden)] # [deprecated (since = "0.22.0" , note = "please use `#[derive(IntoStaticStr)]` instead")] pub fn as_static_str (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let ast = syn :: parse_macro_input ! (input as DeriveInput) ; let toks = macros :: as_ref_str :: as_static_str_inner (& ast , & macros :: as_ref_str :: GenerateTraitVariant :: AsStaticStr ,) . unwrap_or_else (| err | err . to_compile_error ()) ; debug_print_generated (& ast , & toks) ; toks . into () }
};
}
