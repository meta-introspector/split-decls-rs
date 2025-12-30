// Generated macro for check_url_code_point (function)
macro_rules! Depcrate_parsercheck_url_code_point {
() => {
// Module: crate::parser
// Provides: {"check_url_code_point"}
// Dependencies: {}
fn check_url_code_point (vfn : & dyn Fn (SyntaxViolation) , c : char , input : & Input < '_ >) { if c == '%' { let mut input = input . clone () ; if ! matches ! ((input . next () , input . next ()) , (Some (a) , Some (b)) if a . is_ascii_hexdigit () && b . is_ascii_hexdigit ()) { vfn (SyntaxViolation :: PercentDecode) } } else if ! is_url_code_point (c) { vfn (SyntaxViolation :: NonUrlCodePoint) } }
};
}
