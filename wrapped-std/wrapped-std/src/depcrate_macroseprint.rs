// Generated macro for eprint (macro)
macro_rules! Depcrate_macroseprint {
() => {
// Module: crate::macros
// Provides: {"eprint"}
// Dependencies: {}
# [doc = " Prints to the standard error."] # [doc = ""] # [doc = " Equivalent to the [`print!`] macro, except that output goes to"] # [doc = " [`io::stderr`] instead of [`io::stdout`]. See [`print!`] for"] # [doc = " example usage."] # [doc = ""] # [doc = " Use `eprint!` only for error and progress messages. Use `print!`"] # [doc = " instead for the primary output of your program."] # [doc = ""] # [doc = " [`io::stderr`]: crate::io::stderr"] # [doc = " [`io::stdout`]: crate::io::stdout"] # [doc = ""] # [doc = " See the formatting documentation in [`std::fmt`](crate::fmt)"] # [doc = " for details of the macro argument syntax."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if writing to `io::stderr` fails."] # [doc = ""] # [doc = " Writing to non-blocking stderr can cause an error, which will lead"] # [doc = " this macro to panic."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " eprint!(\"Error: Could not complete task\");"] # [doc = " ```"] # [macro_export] # [stable (feature = "eprint" , since = "1.19.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "eprint_macro")] # [allow_internal_unstable (print_internals)] macro_rules ! eprint { ($ ($ arg : tt) *) => { { $ crate :: io :: _eprint ($ crate :: format_args ! ($ ($ arg) *)) ; } } ; }
};
}
