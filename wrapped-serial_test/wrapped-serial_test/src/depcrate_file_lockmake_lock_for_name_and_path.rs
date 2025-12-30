// Generated macro for make_lock_for_name_and_path (function)
macro_rules! Depcrate_file_lockmake_lock_for_name_and_path {
() => {
// Module: crate::file_lock
// Provides: {"make_lock_for_name_and_path"}
// Dependencies: {}
fn make_lock_for_name_and_path (name : & str , path_str : Option < & str >) -> Lock { if let Some (opt_path) = path_str { # [cfg (feature = "logging")] { let path = Path :: new (opt_path) ; if ! path . is_absolute () { debug ! ("Non-absolute path {opt_path} becomes {:?}" , path . canonicalize () . unwrap ()) ; } } Lock :: new (opt_path) } else { let default_path = path_for_name (name) ; Lock :: new (& default_path) } }
};
}
