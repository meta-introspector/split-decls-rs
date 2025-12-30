// Generated macro for RenamedLintFromCommandLine (struct)
macro_rules! Depcrate_lintsRenamedLintFromCommandLine {
() => {
// Module: crate::lints
// Provides: {"RenamedLintFromCommandLine"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_renamed_lint)] pub (crate) struct RenamedLintFromCommandLine < 'a > { pub name : & 'a str , pub replace : & 'a str , # [subdiagnostic] pub suggestion : RenamedLintSuggestion < 'a > , # [subdiagnostic] pub requested_level : RequestedLevel < 'a > , }
};
}
