// Generated macro for path_to_string (function)
macro_rules! Depcrate_expandpath_to_string {
() => {
// Module: crate::expand
// Provides: {"path_to_string"}
// Dependencies: {}
fn path_to_string (path : & Path) -> String { use std :: fmt :: Write ; let mut res = String :: with_capacity (path . segments . len () * 5) ; for i in 0 .. path . segments . len () { write ! (& mut res , "{}" , path . segments [i] . ident) . expect ("writing to a String should never fail") ; if i < path . segments . len () - 1 { res . push_str ("::") ; } } res }
};
}
