// Generated macro for _eprint (function)
macro_rules! Depcrate_io_stdio_eprint {
() => {
// Module: crate::io::stdio
// Provides: {"_eprint"}
// Dependencies: {}
# [unstable (feature = "print_internals" , reason = "implementation detail which may disappear or be replaced at any time" , issue = "none")] # [doc (hidden)] # [cfg (not (test))] pub fn _eprint (args : fmt :: Arguments < '_ >) { print_to (args , stderr , "stderr") ; }
};
}
