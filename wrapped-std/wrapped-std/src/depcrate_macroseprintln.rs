// Generated macro for eprintln (macro)
macro_rules! Depcrate_macroseprintln {
() => {
// Module: crate::macros
// Provides: {"eprintln"}
// Dependencies: {}
# [doc = " Prints to the standard error, with a newline."] # [doc = ""] # [doc = " Equivalent to the [`println!`] macro, except that output goes to"] # [doc = " [`io::stderr`] instead of [`io::stdout`]. See [`println!`] for"] # [doc = " example usage."] # [doc = ""] # [doc = " Use `eprintln!` only for error and progress messages. Use `println!`"] # [doc = " instead for the primary output of your program."] # [doc = ""] # [doc = " See the formatting documentation in [`std::fmt`](crate::fmt)"] # [doc = " for details of the macro argument syntax."] # [doc = ""] # [doc = " [`io::stderr`]: crate::io::stderr"] # [doc = " [`io::stdout`]: crate::io::stdout"] # [doc = " [`println!`]: crate::println"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if writing to `io::stderr` fails."] # [doc = ""] # [doc = " Writing to non-blocking stderr can cause an error, which will lead"] # [doc = " this macro to panic."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " eprintln!(\"Error: Could not complete task\");"] # [doc = " ```"] # [macro_export] # [stable (feature = "eprint" , since = "1.19.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "eprintln_macro")] # [allow_internal_unstable (print_internals , format_args_nl)] macro_rules ! eprintln { () => { $ crate :: eprint ! ("\n") } ; ($ ($ arg : tt) *) => { { $ crate :: io :: _eprint ($ crate :: format_args_nl ! ($ ($ arg) *)) ; } } ; }
};
}
