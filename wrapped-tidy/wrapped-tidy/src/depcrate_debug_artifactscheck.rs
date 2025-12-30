// Generated macro for check (function)
macro_rules! Depcrate_debug_artifactscheck {
() => {
// Module: crate::debug_artifacts
// Provides: {"check"}
// Dependencies: {}
pub fn check (test_dir : & Path , bad : & mut bool) { walk (test_dir , | path , _is_dir | filter_dirs (path) || filter_not_rust (path) , & mut | entry , contents | { for (i , line) in contents . lines () . enumerate () { if line . contains ("borrowck_graphviz_postflow") { tidy_error ! (bad , "{}:{}: {}" , entry . path () . display () , i + 1 , GRAPHVIZ_POSTFLOW_MSG) ; } } } ,) ; }
};
}
