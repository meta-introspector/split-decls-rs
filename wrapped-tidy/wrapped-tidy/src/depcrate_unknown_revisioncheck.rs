// Generated macro for check (function)
macro_rules! Depcrate_unknown_revisioncheck {
() => {
// Module: crate::unknown_revision
// Provides: {"check"}
// Dependencies: {}
pub fn check (tests_path : impl AsRef < Path > , bad : & mut bool) { walk (tests_path . as_ref () , | path , is_dir | { filter_dirs (path) || filter_not_rust (path) || { is_dir && path . file_name () . is_some_and (| name | name == "auxiliary") } } , & mut | entry , contents | visit_test_file (entry , contents , bad) ,) ; }
};
}
