// Generated macro for variant_names (function)
macro_rules! Depcratevariant_names {
() => {
// Module: crate
// Provides: {"variant_names"}
// Dependencies: {}
# [doc = " Implements `Strum::VariantNames` which adds an associated constant `VARIANTS` which is a `'static` slice of discriminant names."] # [doc = ""] # [doc = " Adds an `impl` block for the `enum` that adds a static `VARIANTS` array of `&'static str` that are the discriminant names."] # [doc = " This will respect the `serialize_all` attribute on the `enum` (like `#[strum(serialize_all = \"snake_case\")]`."] # [doc = ""] # [doc = " ```"] # [doc = " // import the macros needed"] # [doc = " use strum_macros::{EnumString};"] # [doc = " // You need to import the trait, to have access to VARIANTS"] # [doc = " use strum::VariantNames;"] # [doc = ""] # [doc = " #[derive(Debug, EnumString, strum_macros::VariantNames)]"] # [doc = " #[strum(serialize_all = \"kebab-case\")]"] # [doc = " enum Color {"] # [doc = "     Red,"] # [doc = "     Blue,"] # [doc = "     Yellow,"] # [doc = "     RebeccaPurple,"] # [doc = " }"] # [doc = " assert_eq!([\"red\", \"blue\", \"yellow\", \"rebecca-purple\"], Color::VARIANTS);"] # [doc = " ```"] # [proc_macro_derive (VariantNames , attributes (strum))] pub fn variant_names (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let ast = syn :: parse_macro_input ! (input as DeriveInput) ; let toks = macros :: enum_variant_names :: enum_variant_names_inner (& ast) . unwrap_or_else (| err | err . to_compile_error ()) ; debug_print_generated (& ast , & toks) ; toks . into () }
};
}
