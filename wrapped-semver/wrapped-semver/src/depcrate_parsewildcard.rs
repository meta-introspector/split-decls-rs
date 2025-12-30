// Generated macro for wildcard (function)
macro_rules! Depcrate_parsewildcard {
() => {
// Module: crate::parse
// Provides: {"wildcard"}
// Dependencies: {}
fn wildcard (input : & str) -> Option < (char , & str) > { if let Some (rest) = input . strip_prefix ('*') { Some (('*' , rest)) } else if let Some (rest) = input . strip_prefix ('x') { Some (('x' , rest)) } else if let Some (rest) = input . strip_prefix ('X') { Some (('X' , rest)) } else { None } }
};
}
