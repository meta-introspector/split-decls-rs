// Generated macro for split_os_string_arg (function)
macro_rules! Depcrate_compiler_argssplit_os_string_arg {
() => {
// Module: crate::compiler::args
// Provides: {"split_os_string_arg"}
// Dependencies: {}
pub fn split_os_string_arg (val : OsString , split : & str) -> ArgParseResult < (String , Option < String >) > { let val = val . into_string () . map_err (ArgParseError :: InvalidUnicode) ? ; let mut split_it = val . splitn (2 , split) ; let s1 = split_it . next () . expect ("splitn with no values") ; let maybe_s2 = split_it . next () ; Ok ((s1 . to_owned () , maybe_s2 . map (| s | s . to_owned ()))) }
};
}
