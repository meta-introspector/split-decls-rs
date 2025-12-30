// Generated macro for OverflowingInt (struct)
macro_rules! Depcrate_lintsOverflowingInt {
() => {
// Module: crate::lints
// Provides: {"OverflowingInt"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_overflowing_int)] # [note] pub (crate) struct OverflowingInt < 'a > { pub ty : & 'a str , pub lit : String , pub min : i128 , pub max : u128 , # [subdiagnostic] pub help : Option < OverflowingIntHelp < 'a > > , }
};
}
