// Generated macro for summarize_diags (function)
macro_rules! Depcrate_parser_testssummarize_diags {
() => {
// Module: crate::parser::tests
// Provides: {"summarize_diags"}
// Dependencies: {}
fn summarize_diags (diags : ParseDiags) -> Vec < (String , miette :: SourceSpan) > { diags . into_iter () . map (| diag | { let code = diag . code () . expect ("Diagnostic should have a code") . to_string () ; let span = * diag . span () ; (code , span) }) . collect () }
};
}
