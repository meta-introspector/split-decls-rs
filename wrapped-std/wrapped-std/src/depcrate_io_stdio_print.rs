// Generated macro for _print (function)
macro_rules! Depcrate_io_stdio_print {
() => {
// Module: crate::io::stdio
// Provides: {"_print"}
// Dependencies: {}
# [unstable (feature = "print_internals" , reason = "implementation detail which may disappear or be replaced at any time" , issue = "none")] # [doc (hidden)] # [cfg (not (test))] pub fn _print (args : fmt :: Arguments < '_ >) { print_to (args , stdout , "stdout") ; }
};
}
