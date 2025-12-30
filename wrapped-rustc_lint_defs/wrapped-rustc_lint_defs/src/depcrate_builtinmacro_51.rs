// Generated macro for macro_51 (macro)
macro_rules! Depcrate_builtinmacro_51 {
() => {
// Module: crate::builtin
// Provides: {"macro_51"}
// Dependencies: {}
declare_lint ! { # [doc = " The `pub_use_of_private_extern_crate` lint detects a specific"] # [doc = " situation of re-exporting a private `extern crate`."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " extern crate core;"] # [doc = " pub use core as reexported_core;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " A public `use` declaration should not be used to publically re-export a"] # [doc = " private `extern crate`. `pub extern crate` should be used instead."] # [doc = ""] # [doc = " This was historically allowed, but is not the intended behavior"] # [doc = " according to the visibility rules. This is a [future-incompatible]"] # [doc = " lint to transition this to a hard error in the future. See [issue"] # [doc = " #127909] for more details."] # [doc = ""] # [doc = " [issue #127909]: https://github.com/rust-lang/rust/issues/127909"] # [doc = " [future-incompatible]: ../index.md#future-incompatible-lints"] pub PUB_USE_OF_PRIVATE_EXTERN_CRATE , Deny , "detect public re-exports of private extern crates" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "issue #127909 <https://github.com/rust-lang/rust/issues/127909>" , report_in_deps : true , } ; }
};
}
