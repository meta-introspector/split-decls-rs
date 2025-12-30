// Generated macro for path (function)
macro_rules! Depcrate_gzpath {
() => {
// Module: crate::gz
// Provides: {"path"}
// Dependencies: {}
fn path (prefix : & Path , file : & str) -> String { let mut path_buf = prefix . to_path_buf () ; path_buf . push (file) ; path_buf . as_path () . to_str () . unwrap () . to_owned () }
};
}
