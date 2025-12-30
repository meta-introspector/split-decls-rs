// Generated macro for to_camel_case (function)
macro_rules! Depcrate_proptest_fnto_camel_case {
() => {
// Module: crate::proptest_fn
// Provides: {"to_camel_case"}
// Dependencies: {}
fn to_camel_case (s : & str) -> String { let mut upper = true ; let mut r = String :: new () ; for c in s . chars () { if c == '_' { upper = true ; } else if upper { r . push_str (& c . to_uppercase () . to_string ()) ; upper = false ; } else { r . push (c) ; } } r }
};
}
