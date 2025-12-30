// Generated macro for macro_121 (macro)
macro_rules! Depcrate_builtinmacro_121 {
() => {
// Module: crate::builtin
// Provides: {"macro_121"}
// Dependencies: {}
declare_lint ! { # [doc = " The `test_unstable_lint` lint tests unstable lints and is perma-unstable."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " // This lint is intentionally used to test the compiler's behavior"] # [doc = " // when an unstable lint is enabled without the corresponding feature gate."] # [doc = " #![allow(test_unstable_lint)]"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " In order to test the behavior of unstable lints, a permanently-unstable"] # [doc = " lint is required. This lint can be used to trigger warnings and errors"] # [doc = " from the compiler related to unstable lints."] pub TEST_UNSTABLE_LINT , Deny , "this unstable lint is only for testing" , @ feature_gate = test_unstable_lint ; }
};
}
