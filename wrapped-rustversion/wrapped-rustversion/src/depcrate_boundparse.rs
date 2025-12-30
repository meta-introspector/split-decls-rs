// Generated macro for parse (function)
macro_rules! Depcrate_boundparse {
() => {
// Module: crate::bound
// Provides: {"parse"}
// Dependencies: {}
pub fn parse (paren : Group , iter : Iter) -> Result < Bound > { if let Some (TokenTree :: Literal (literal)) = iter . peek () { let repr = literal . to_string () ; if repr . starts_with (| ch : char | ch . is_ascii_digit ()) { if repr . contains ('.') { return release :: parse (paren , iter) . map (Bound :: Stable) ; } else { return date :: parse (paren , iter) . map (Bound :: Nightly) ; } } } let msg = format ! ("expected rustc release number like 1.85, or nightly date like {}" , time :: today () ,) ; Err (Error :: group (paren , msg)) }
};
}
