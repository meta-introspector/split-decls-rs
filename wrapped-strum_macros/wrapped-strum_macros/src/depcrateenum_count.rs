// Generated macro for enum_count (function)
macro_rules! Depcrateenum_count {
() => {
// Module: crate
// Provides: {"enum_count"}
// Dependencies: {}
# [doc = " Add a constant `usize` equal to the number of variants."] # [doc = ""] # [doc = " For a given enum generates implementation of `strum::EnumCount`,"] # [doc = " which adds a static property `COUNT` of type usize that holds the number of variants."] # [doc = ""] # [doc = " ```"] # [doc = " use strum::{EnumCount, IntoEnumIterator};"] # [doc = " use strum_macros::{EnumCount as EnumCountMacro, EnumIter};"] # [doc = ""] # [doc = " #[derive(Debug, EnumCountMacro, EnumIter)]"] # [doc = " enum Week {"] # [doc = "     Sunday,"] # [doc = "     Monday,"] # [doc = "     Tuesday,"] # [doc = "     Wednesday,"] # [doc = "     Thursday,"] # [doc = "     Friday,"] # [doc = "     Saturday,"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(7, Week::COUNT);"] # [doc = " assert_eq!(Week::iter().count(), Week::COUNT);"] # [doc = ""] # [doc = " ```"] # [proc_macro_derive (EnumCount , attributes (strum))] pub fn enum_count (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let ast = syn :: parse_macro_input ! (input as DeriveInput) ; let toks = macros :: enum_count :: enum_count_inner (& ast) . unwrap_or_else (| err | err . to_compile_error ()) ; debug_print_generated (& ast , & toks) ; toks . into () }
};
}
