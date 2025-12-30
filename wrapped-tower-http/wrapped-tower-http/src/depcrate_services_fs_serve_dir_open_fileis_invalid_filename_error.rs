// Generated macro for is_invalid_filename_error (function)
macro_rules! Depcrate_services_fs_serve_dir_open_fileis_invalid_filename_error {
() => {
// Module: crate::services::fs::serve_dir::open_file
// Provides: {"is_invalid_filename_error"}
// Dependencies: {}
fn is_invalid_filename_error (err : & io :: Error) -> bool { if err . kind () == ErrorKind :: InvalidInput { return true ; } # [cfg (windows)] if let Some (raw_err) = err . raw_os_error () { if (raw_err == 123) || (raw_err == 161) || (raw_err == 206) { return true ; } } false }
};
}
