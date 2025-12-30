// Generated macro for macro_18 (macro)
macro_rules! Depcrate_builtinmacro_18 {
() => {
// Module: crate::builtin
// Provides: {"macro_18"}
// Dependencies: {}
declare_lint ! { # [doc = " The `forbidden_lint_groups` lint detects violations of"] # [doc = " `forbid` applied to a lint group. Due to a bug in the compiler,"] # [doc = " these used to be overlooked entirely. They now generate a warning."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![forbid(warnings)]"] # [doc = " #![warn(bad_style)]"] # [doc = ""] # [doc = " fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Recommended fix"] # [doc = ""] # [doc = " If your crate is using `#![forbid(warnings)]`,"] # [doc = " we recommend that you change to `#![deny(warnings)]`."] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Due to a compiler bug, applying `forbid` to lint groups"] # [doc = " previously had no effect. The bug is now fixed but instead of"] # [doc = " enforcing `forbid` we issue this future-compatibility warning"] # [doc = " to avoid breaking existing crates."] pub FORBIDDEN_LINT_GROUPS , Warn , "applying forbid to lint-groups" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "issue #81670 <https://github.com/rust-lang/rust/issues/81670>" , report_in_deps : true , } ; }
};
}
