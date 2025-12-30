// Generated macro for NonExhaustiveOmittedPatternLintOnArm (struct)
macro_rules! Depcrate_errorsNonExhaustiveOmittedPatternLintOnArm {
() => {
// Module: crate::errors
// Provides: {"NonExhaustiveOmittedPatternLintOnArm"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (pattern_analysis_non_exhaustive_omitted_pattern_lint_on_arm)] # [help] pub (crate) struct NonExhaustiveOmittedPatternLintOnArm { # [label] pub lint_span : Span , # [suggestion (code = "#[{lint_level}({lint_name})]\n" , applicability = "maybe-incorrect")] pub suggest_lint_on_match : Option < Span > , pub lint_level : & 'static str , pub lint_name : & 'static str , }
};
}
