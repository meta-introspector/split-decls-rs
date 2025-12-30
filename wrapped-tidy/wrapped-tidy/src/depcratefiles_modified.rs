// Generated macro for files_modified (function)
macro_rules! Depcratefiles_modified {
() => {
// Module: crate
// Provides: {"files_modified"}
// Dependencies: {}
# [doc = " Returns true if any modified file matches the predicate, if we are in CI, or if unable to list modified files."] pub fn files_modified (ci_info : & CiInfo , pred : impl Fn (& str) -> bool) -> bool { let mut v = vec ! [()] ; files_modified_batch_filter (ci_info , & mut v , | _ , p | pred (p)) ; ! v . is_empty () }
};
}
