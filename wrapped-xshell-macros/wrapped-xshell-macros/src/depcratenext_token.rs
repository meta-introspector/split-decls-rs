// Generated macro for next_token (function)
macro_rules! Depcratenext_token {
() => {
// Module: crate
// Provides: {"next_token"}
// Dependencies: {}
fn next_token (s : & str) -> Result < (usize , TokenKind) > { if s . starts_with ('{') { let len = s . find ('}') . ok_or_else (| | "unclosed `{` in command" . to_string ()) ? + 1 ; let splat = s [.. len] . ends_with ("...}") ; return Ok ((len , TokenKind :: Interpolation { splat })) ; } if s . starts_with ('\'') { let len = s [1 ..] . find ('\'') . ok_or_else (| | "unclosed `'` in command" . to_string ()) ? + 2 ; return Ok ((len , TokenKind :: String)) ; } let len = s . find (| it : char | it . is_ascii_whitespace () || it == '\'' || it == '{') . unwrap_or (s . len ()) ; Ok ((len , TokenKind :: Word)) }
};
}
