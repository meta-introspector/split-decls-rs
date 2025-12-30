// Generated macro for path_for_name (function)
macro_rules! Depcrate_file_lockpath_for_name {
() => {
// Module: crate::file_lock
// Provides: {"path_for_name"}
// Dependencies: {}
pub (crate) fn path_for_name (name : & str) -> String { let mut pathbuf = env :: temp_dir () ; pathbuf . push (format ! ("serial-test-{}" , name)) ; pathbuf . into_os_string () . into_string () . unwrap () }
};
}
