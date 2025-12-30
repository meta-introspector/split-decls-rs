// Generated macro for UnknownDiagnosticAttribute (struct)
macro_rules! Depcrate_lintsUnknownDiagnosticAttribute {
() => {
// Module: crate::lints
// Provides: {"UnknownDiagnosticAttribute"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (lint_unknown_diagnostic_attribute)] pub (crate) struct UnknownDiagnosticAttribute { # [subdiagnostic] pub typo : Option < UnknownDiagnosticAttributeTypoSugg > , }
};
}
