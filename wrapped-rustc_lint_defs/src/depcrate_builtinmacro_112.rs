// Generated macro for macro_112 (macro)
macro_rules! Depcrate_builtinmacro_112 {
() => {
// Module: crate::builtin
// Provides: {"macro_112"}
// Dependencies: {}
declare_lint ! { # [doc = " The `rust_2021_prefixes_incompatible_syntax` lint detects identifiers that will be parsed as a"] # [doc = " prefix instead in Rust 2021."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,edition2018,compile_fail"] # [doc = " #![deny(rust_2021_prefixes_incompatible_syntax)]"] # [doc = ""] # [doc = " macro_rules! m {"] # [doc = "     (z $x:expr) => ();"] # [doc = " }"] # [doc = ""] # [doc = " m!(z\"hey\");"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " In Rust 2015 and 2018, `z\"hey\"` is two tokens: the identifier `z`"] # [doc = " followed by the string literal `\"hey\"`. In Rust 2021, the `z` is"] # [doc = " considered a prefix for `\"hey\"`."] # [doc = ""] # [doc = " This lint suggests to add whitespace between the `z` and `\"hey\"` tokens"] # [doc = " to keep them separated in Rust 2021."] # [allow (rustdoc :: invalid_rust_codeblocks)] pub RUST_2021_PREFIXES_INCOMPATIBLE_SYNTAX , Allow , "identifiers that will be parsed as a prefix in Rust 2021" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: EditionError (Edition :: Edition2021) , reference : "<https://doc.rust-lang.org/edition-guide/rust-2021/reserving-syntax.html>" , } ; crate_level_only }
};
}
