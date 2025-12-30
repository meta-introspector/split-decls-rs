// Generated macro for enum_is (function)
macro_rules! Depcrateenum_is {
() => {
// Module: crate
// Provides: {"enum_is"}
// Dependencies: {}
# [doc = " Generated `is_*()` methods for each variant."] # [doc = " E.g. `Color.is_red()`."] # [doc = ""] # [doc = " ```"] # [doc = ""] # [doc = " use strum_macros::EnumIs;"] # [doc = ""] # [doc = " #[derive(EnumIs, Debug)]"] # [doc = " enum Color {"] # [doc = "     Red,"] # [doc = "     Green { range: usize },"] # [doc = " }"] # [doc = ""] # [doc = " assert!(Color::Red.is_red());"] # [doc = " assert!(Color::Green{range: 0}.is_green());"] # [doc = " ```"] # [proc_macro_derive (EnumIs , attributes (strum))] pub fn enum_is (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let ast = syn :: parse_macro_input ! (input as DeriveInput) ; let toks = macros :: enum_is :: enum_is_inner (& ast) . unwrap_or_else (| err | err . to_compile_error ()) ; debug_print_generated (& ast , & toks) ; toks . into () }
};
}
