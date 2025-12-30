// Generated macro for preferred_encoding (function)
macro_rules! Depcrate_services_fs_serve_dir_open_filepreferred_encoding {
() => {
// Module: crate::services::fs::serve_dir::open_file
// Provides: {"preferred_encoding"}
// Dependencies: {}
fn preferred_encoding (path : & mut PathBuf , negotiated_encoding : & [(Encoding , QValue)] ,) -> Option < Encoding > { let preferred_encoding = Encoding :: preferred_encoding (negotiated_encoding . iter () . copied ()) ; if let Some (file_extension) = preferred_encoding . and_then (| encoding | encoding . to_file_extension ()) { let new_file_name = path . file_name () . map (| file_name | { let mut os_string = file_name . to_os_string () ; os_string . push (file_extension) ; os_string }) . unwrap_or_else (| | file_extension . to_os_string ()) ; path . set_file_name (new_file_name) ; } preferred_encoding }
};
}
