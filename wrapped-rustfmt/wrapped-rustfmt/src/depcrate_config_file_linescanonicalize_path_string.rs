// Generated macro for canonicalize_path_string (function)
macro_rules! Depcrate_config_file_linescanonicalize_path_string {
() => {
// Module: crate::config::file_lines
// Provides: {"canonicalize_path_string"}
// Dependencies: {}
fn canonicalize_path_string (file : & FileName) -> Option < FileName > { match * file { FileName :: Real (ref path) => path . canonicalize () . ok () . map (FileName :: Real) , _ => Some (file . clone ()) , } }
};
}
