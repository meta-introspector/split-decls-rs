// Generated macro for previous_non_trivia_token (function)
macro_rules! Depcrate_algoprevious_non_trivia_token {
() => {
// Module: crate::algo
// Provides: {"previous_non_trivia_token"}
// Dependencies: {}
pub fn previous_non_trivia_token (e : impl Into < SyntaxElement >) -> Option < SyntaxToken > { let mut token = match e . into () { SyntaxElement :: Node (n) => n . first_token () ? , SyntaxElement :: Token (t) => t , } . prev_token () ; while let Some (inner) = token { if ! inner . kind () . is_trivia () { return Some (inner) ; } else { token = inner . prev_token () ; } } None }
};
}
