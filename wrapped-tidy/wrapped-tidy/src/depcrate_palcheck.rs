// Generated macro for check (function)
macro_rules! Depcrate_palcheck {
() => {
// Module: crate::pal
// Provides: {"check"}
// Dependencies: {}
pub fn check (path : & Path , bad : & mut bool) { let mut saw_target_arch = false ; let mut saw_cfg_bang = false ; walk (path , | path , _is_dir | filter_dirs (path) , & mut | entry , contents | { let file = entry . path () ; let filestr = file . to_string_lossy () . replace ("\\" , "/") ; if ! filestr . ends_with (".rs") { return ; } let is_exception_path = EXCEPTION_PATHS . iter () . any (| s | filestr . contains (& * * s)) ; if is_exception_path { return ; } if filestr . contains ("tests") || filestr . contains ("benches") { return ; } check_cfgs (contents , file , bad , & mut saw_target_arch , & mut saw_cfg_bang) ; }) ; assert ! (saw_target_arch) ; assert ! (saw_cfg_bang) ; }
};
}
