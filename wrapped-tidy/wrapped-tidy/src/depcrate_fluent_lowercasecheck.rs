// Generated macro for check (function)
macro_rules! Depcrate_fluent_lowercasecheck {
() => {
// Module: crate::fluent_lowercase
// Provides: {"check"}
// Dependencies: {}
pub fn check (path : & Path , bad : & mut bool) { walk (path , | path , is_dir | filter_dirs (path) || (! is_dir && filter_fluent (path)) , & mut | ent , contents | { check_lowercase (ent . path () . to_str () . unwrap () , contents , bad) ; } ,) ; }
};
}
