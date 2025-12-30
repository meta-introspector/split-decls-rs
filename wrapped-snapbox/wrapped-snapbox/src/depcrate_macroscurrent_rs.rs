// Generated macro for current_rs (macro)
macro_rules! Depcrate_macroscurrent_rs {
() => {
// Module: crate::macros
// Provides: {"current_rs"}
// Dependencies: {}
# [doc = " Find the directory for your source file"] # [doc (hidden)] # [macro_export] macro_rules ! current_rs { () => { { let root = $ crate :: utils :: cargo_rustc_current_dir ! () ; let file = :: std :: file ! () ; let rel_path = :: std :: path :: Path :: new (file) ; root . join (rel_path) } } ; }
};
}
