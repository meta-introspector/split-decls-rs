// Generated macro for is_dir (function)
macro_rules! Depcrate_services_fs_serve_dir_open_fileis_dir {
() => {
// Module: crate::services::fs::serve_dir::open_file
// Provides: {"is_dir"}
// Dependencies: {}
async fn is_dir (path_to_file : & Path) -> bool { tokio :: fs :: metadata (path_to_file) . await . map_or (false , | meta_data | meta_data . is_dir ()) }
};
}
