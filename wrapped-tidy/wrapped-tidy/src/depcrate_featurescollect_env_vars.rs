// Generated macro for collect_env_vars (function)
macro_rules! Depcrate_featurescollect_env_vars {
() => {
// Module: crate::features
// Provides: {"collect_env_vars"}
// Dependencies: {}
pub fn collect_env_vars (compiler : & Path) -> BTreeSet < String > { let env_var_regex : Regex = Regex :: new (r#"env::var(_os)?\("([^"]+)"#) . unwrap () ; let mut vars = BTreeSet :: new () ; walk (compiler , | path , _is_dir | { filter_dirs (path) || filter_not_rust (path) || path . ends_with ("build.rs") || path . ends_with ("tests.rs") } , & mut | _entry , contents | { for env_var in env_var_regex . captures_iter (contents) . map (| c | c . get (2) . unwrap () . as_str ()) { if should_document (env_var) { vars . insert (env_var . to_owned ()) ; } } } ,) ; vars }
};
}
