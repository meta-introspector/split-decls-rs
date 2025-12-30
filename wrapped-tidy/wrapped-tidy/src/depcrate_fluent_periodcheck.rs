// Generated macro for check (function)
macro_rules! Depcrate_fluent_periodcheck {
() => {
// Module: crate::fluent_period
// Provides: {"check"}
// Dependencies: {}
pub fn check (path : & Path , bad : & mut bool) { walk (path , | path , is_dir | filter_dirs (path) || (! is_dir && filter_fluent (path)) , & mut | ent , contents | { check_period (ent . path () . to_str () . unwrap () , contents , bad) ; } ,) ; }
};
}
