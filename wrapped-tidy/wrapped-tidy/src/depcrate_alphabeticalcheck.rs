// Generated macro for check (function)
macro_rules! Depcrate_alphabeticalcheck {
() => {
// Module: crate::alphabetical
// Provides: {"check"}
// Dependencies: {}
pub fn check (path : & Path , bad : & mut bool) { let skip = | path : & _ , _is_dir | filter_dirs (path) || path . ends_with ("tidy/src/alphabetical/tests.rs") ; walk (path , skip , & mut | entry , contents | { let file = & entry . path () . display () ; let lines = contents . lines () . enumerate () ; check_lines (file , lines , & mut crate :: tidy_error , bad) }) ; }
};
}
