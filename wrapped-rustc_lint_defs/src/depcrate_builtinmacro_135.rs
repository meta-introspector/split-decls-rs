// Generated macro for macro_135 (macro)
macro_rules! Depcrate_builtinmacro_135 {
() => {
// Module: crate::builtin
// Provides: {"macro_135"}
// Dependencies: {}
declare_lint ! { # [doc = " The `ambiguous_glob_imports` lint detects glob imports that should report ambiguity"] # [doc = " errors, but previously didn't do that due to rustc bugs."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #![deny(ambiguous_glob_imports)]"] # [doc = " pub fn foo() -> u32 {"] # [doc = "     use sub::*;"] # [doc = "     C"] # [doc = " }"] # [doc = ""] # [doc = " mod sub {"] # [doc = "     mod mod1 { pub const C: u32 = 1; }"] # [doc = "     mod mod2 { pub const C: u32 = 2; }"] # [doc = ""] # [doc = "     pub use mod1::*;"] # [doc = "     pub use mod2::*;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Previous versions of Rust compile it successfully because it"] # [doc = " had lost the ambiguity error when resolve `use sub::mod2::*`."] # [doc = ""] # [doc = " This is a [future-incompatible] lint to transition this to a"] # [doc = " hard error in the future."] # [doc = ""] # [doc = " [future-incompatible]: ../index.md#future-incompatible-lints"] pub AMBIGUOUS_GLOB_IMPORTS , Deny , "detects certain glob imports that require reporting an ambiguity error" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "issue #114095 <https://github.com/rust-lang/rust/issues/114095>" , report_in_deps : true , } ; }
};
}
