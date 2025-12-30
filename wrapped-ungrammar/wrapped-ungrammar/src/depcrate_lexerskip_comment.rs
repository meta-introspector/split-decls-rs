// Generated macro for skip_comment (function)
macro_rules! Depcrate_lexerskip_comment {
() => {
// Module: crate::lexer
// Provides: {"skip_comment"}
// Dependencies: {}
fn skip_comment (input : & mut & str) { if input . starts_with ("//") { let idx = input . find ('\n') . map_or (input . len () , | it | it + 1) ; * input = & input [idx ..] } }
};
}
