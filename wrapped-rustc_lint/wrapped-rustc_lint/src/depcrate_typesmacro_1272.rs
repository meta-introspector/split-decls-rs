// Generated macro for macro_1272 (macro)
macro_rules! Depcrate_typesmacro_1272 {
() => {
// Module: crate::types
// Provides: {"macro_1272"}
// Dependencies: {}
declare_lint ! { # [doc = " The `unused_comparisons` lint detects comparisons made useless by"] # [doc = " limits of the types involved."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn foo(x: u8) {"] # [doc = "     x >= 0;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " A useless comparison may indicate a mistake, and should be fixed or"] # [doc = " removed."] UNUSED_COMPARISONS , Warn , "comparisons made useless by limits of the types involved" }
};
}
