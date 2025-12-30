// Generated macro for to_string (function)
macro_rules! Depcrateto_string {
() => {
// Module: crate
// Provides: {"to_string"}
// Dependencies: {}
# [doc = " implements `std::string::ToString` on an enum"] # [doc = ""] # [doc = " ```"] # [doc = " // You need to bring the ToString trait into scope to use it"] # [doc = " use std::string::ToString;"] # [doc = " use strum_macros;"] # [doc = ""] # [doc = " #[derive(strum_macros::ToString, Debug)]"] # [doc = " enum Color {"] # [doc = "     #[strum(serialize = \"redred\")]"] # [doc = "     Red,"] # [doc = "     Green {"] # [doc = "         range: usize,"] # [doc = "     },"] # [doc = "     Blue(usize),"] # [doc = "     Yellow,"] # [doc = " }"] # [doc = ""] # [doc = " // uses the serialize string for Display"] # [doc = " let red = Color::Red;"] # [doc = " assert_eq!(String::from(\"redred\"), red.to_string());"] # [doc = " // by default the variants Name"] # [doc = " let yellow = Color::Yellow;"] # [doc = " assert_eq!(String::from(\"Yellow\"), yellow.to_string());"] # [doc = " ```"] # [deprecated (since = "0.22.0" , note = "please use `#[derive(Display)]` instead. See issue https://github.com/Peternator7/strum/issues/132")] # [doc (hidden)] # [proc_macro_derive (ToString , attributes (strum))] pub fn to_string (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let ast = syn :: parse_macro_input ! (input as DeriveInput) ; let toks = macros :: to_string :: to_string_inner (& ast) . unwrap_or_else (| err | err . to_compile_error ()) ; debug_print_generated (& ast , & toks) ; toks . into () }
};
}
