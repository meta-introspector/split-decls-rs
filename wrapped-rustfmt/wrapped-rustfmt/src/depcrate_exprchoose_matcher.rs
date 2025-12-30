// Generated macro for choose_matcher (function)
macro_rules! Depcrate_exprchoose_matcher {
() => {
// Module: crate::expr
// Provides: {"choose_matcher"}
// Dependencies: {}
fn choose_matcher (pat : Option < & ast :: Pat >) -> & 'static str { pat . map_or ("" , | _ | "let") }
};
}
