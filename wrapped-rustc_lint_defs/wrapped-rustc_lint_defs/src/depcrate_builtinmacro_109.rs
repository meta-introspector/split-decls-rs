// Generated macro for macro_109 (macro)
macro_rules! Depcrate_builtinmacro_109 {
() => {
// Module: crate::builtin
// Provides: {"macro_109"}
// Dependencies: {}
declare_lint ! { # [doc = " The `rust_2021_incompatible_or_patterns` lint detects usage of old versions of or-patterns."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,edition2018,compile_fail"] # [doc = " #![deny(rust_2021_incompatible_or_patterns)]"] # [doc = ""] # [doc = " macro_rules! match_any {"] # [doc = "     ( $expr:expr , $( $( $pat:pat )|+ => $expr_arm:expr ),+ ) => {"] # [doc = "         match $expr {"] # [doc = "             $("] # [doc = "                 $( $pat => $expr_arm, )+"] # [doc = "             )+"] # [doc = "         }"] # [doc = "     };"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let result: Result<i64, i32> = Err(42);"] # [doc = "     let int: i64 = match_any!(result, Ok(i) | Err(i) => i.into());"] # [doc = "     assert_eq!(int, 42);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " In Rust 2021, the `pat` matcher will match additional patterns, which include the `|` character."] pub RUST_2021_INCOMPATIBLE_OR_PATTERNS , Allow , "detects usage of old versions of or-patterns" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: EditionError (Edition :: Edition2021) , reference : "<https://doc.rust-lang.org/edition-guide/rust-2021/or-patterns-macro-rules.html>" , } ; }
};
}
