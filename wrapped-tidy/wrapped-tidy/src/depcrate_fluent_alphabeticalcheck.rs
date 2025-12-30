// Generated macro for check (function)
macro_rules! Depcrate_fluent_alphabeticalcheck {
() => {
// Module: crate::fluent_alphabetical
// Provides: {"check"}
// Dependencies: {}
pub fn check (path : & Path , bless : bool , bad : & mut bool) { let mut all_defined_msgs = HashMap :: new () ; walk (path , | path , is_dir | filter_dirs (path) || (! is_dir && ! is_fluent (path)) , & mut | ent , contents | { if bless { let sorted = sort_messages (ent . path () . to_str () . unwrap () , contents , bad , & mut all_defined_msgs ,) ; if sorted != contents { let mut f = OpenOptions :: new () . write (true) . truncate (true) . open (ent . path ()) . unwrap () ; f . write_all (sorted . as_bytes ()) . unwrap () ; } } else { check_alphabetic (ent . path () . to_str () . unwrap () , contents , bad , & mut all_defined_msgs ,) ; } } ,) ; crate :: fluent_used :: check (path , all_defined_msgs , bad) ; }
};
}
