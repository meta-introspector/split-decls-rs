// Generated macro for Termination (trait)
macro_rules! Depcrate_processTermination {
() => {
// Module: crate::process
// Provides: {"Termination"}
// Dependencies: {}
# [doc = " A trait for implementing arbitrary return types in the `main` function."] # [doc = ""] # [doc = " The C-main function only supports returning integers."] # [doc = " So, every type implementing the `Termination` trait has to be converted"] # [doc = " to an integer."] # [doc = ""] # [doc = " The default implementations are returning `libc::EXIT_SUCCESS` to indicate"] # [doc = " a successful execution. In case of a failure, `libc::EXIT_FAILURE` is returned."] # [doc = ""] # [doc = " Because different runtimes have different specifications on the return value"] # [doc = " of the `main` function, this trait is likely to be available only on"] # [doc = " standard library's runtime for convenience. Other runtimes are not required"] # [doc = " to provide similar functionality."] # [cfg_attr (not (any (test , doctest)) , lang = "termination")] # [stable (feature = "termination_trait_lib" , since = "1.61.0")] # [rustc_on_unimplemented (on (cause = "MainFunctionType" , message = "`main` has invalid return type `{Self}`" , label = "`main` can only return types that implement `{This}`"))] pub trait Termination { # [doc = " Is called to get the representation of the value as status code."] # [doc = " This status code is returned to the operating system."] # [stable (feature = "termination_trait_lib" , since = "1.61.0")] fn report (self) -> ExitCode ; }
};
}
