// Generated macro for current_dir (macro)
macro_rules! Depcrate_macroscurrent_dir {
() => {
// Module: crate::macros
// Provides: {"current_dir"}
// Dependencies: {}
# [doc = " Find the directory for your source file"] # [doc (hidden)] # [macro_export] macro_rules ! current_dir { () => { { let root = $ crate :: utils :: cargo_rustc_current_dir ! () ; let file = :: std :: file ! () ; let rel_path = :: std :: path :: Path :: new (file) . parent () . unwrap () ; root . join (rel_path) } } ; }
};
}
