// Generated macro for read_temp_dir (function)
macro_rules! Depcrate_unix_linux_componentread_temp_dir {
() => {
// Module: crate::unix::linux::component
// Provides: {"read_temp_dir"}
// Dependencies: {}
fn read_temp_dir < F : FnMut (PathBuf) > (path : & Path , starts_with : & str , mut f : F) { if let Ok (dir) = read_dir (path) { for entry in dir . flatten () { if ! entry . file_name () . to_str () . unwrap_or ("") . starts_with (starts_with) { continue ; } let path = entry . path () ; if ! path . is_file () { f (path) ; } } } }
};
}
