// Generated macro for macro_46 (macro)
macro_rules! Depcrate_builtinmacro_46 {
() => {
// Module: crate::builtin
// Provides: {"macro_46"}
// Dependencies: {}
declare_lint ! { # [doc = " The `stable_features` lint detects a [`feature` attribute] that"] # [doc = " has since been made stable."] # [doc = ""] # [doc = " [`feature` attribute]: https://doc.rust-lang.org/nightly/unstable-book/"] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![feature(test_accepted_feature)]"] # [doc = " fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " When a feature is stabilized, it is no longer necessary to include a"] # [doc = " `#![feature]` attribute for it. To fix, simply remove the"] # [doc = " `#![feature]` attribute."] pub STABLE_FEATURES , Warn , "stable features found in `#[feature]` directive" }
};
}
