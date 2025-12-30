// Generated macro for OnlyCastu8ToChar (struct)
macro_rules! Depcrate_lintsOnlyCastu8ToChar {
() => {
// Module: crate::lints
// Provides: {"OnlyCastu8ToChar"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_only_cast_u8_to_char)] pub (crate) struct OnlyCastu8ToChar { # [suggestion (code = "'\\u{{{literal:X}}}'" , applicability = "machine-applicable")] pub span : Span , pub literal : u128 , }
};
}
