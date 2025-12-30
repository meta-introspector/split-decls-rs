// Generated macro for check_runtime_no_proc_macros (function)
macro_rules! Depcrate_depscheck_runtime_no_proc_macros {
() => {
// Module: crate::deps
// Provides: {"check_runtime_no_proc_macros"}
// Dependencies: {}
fn check_runtime_no_proc_macros (metadata : & Metadata , bad : & mut bool) { for pkg in & metadata . packages { if pkg . targets . iter () . any (| target | target . is_proc_macro ()) { tidy_error ! (bad , "proc macro `{}` is not allowed as standard library dependency.\n\
                Using proc macros in the standard library would break cross-compilation \
                as proc-macros don't get shipped for the host tuple." , pkg . name) ; } } }
};
}
