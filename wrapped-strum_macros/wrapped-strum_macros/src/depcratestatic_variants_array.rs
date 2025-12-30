// Generated macro for static_variants_array (function)
macro_rules! Depcratestatic_variants_array {
() => {
// Module: crate
// Provides: {"static_variants_array"}
// Dependencies: {}
# [doc = " Adds a `'static` slice with all of the Enum's variants."] # [doc = ""] # [doc = " Implements `strum::VariantArray` which adds an associated constant `VARIANTS`."] # [doc = " This constant contains an array with all the variants of the enumerator."] # [doc = ""] # [doc = " This trait can only be autoderived if the enumerator is composed only of unit-type variants,"] # [doc = " meaning that the variants must not have any data."] # [doc = ""] # [doc = " ```"] # [doc = " use strum::VariantArray as _;"] # [doc = " use strum_macros::VariantArray;"] # [doc = ""] # [doc = " #[derive(VariantArray, Debug, PartialEq, Eq)]"] # [doc = " enum Op {"] # [doc = "     Add,"] # [doc = "     Sub,"] # [doc = "     Mul,"] # [doc = "     Div,"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(Op::VARIANTS, &[Op::Add, Op::Sub, Op::Mul, Op::Div]);"] # [doc = " ```"] # [proc_macro_derive (VariantArray , attributes (strum))] pub fn static_variants_array (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let ast = syn :: parse_macro_input ! (input as DeriveInput) ; let toks = macros :: enum_variant_array :: static_variants_array_inner (& ast) . unwrap_or_else (| err | err . to_compile_error ()) ; debug_print_generated (& ast , & toks) ; toks . into () }
};
}
