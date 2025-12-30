// Generated macro for maybe_redirect_or_append_path (function)
macro_rules! Depcrate_services_fs_serve_dir_open_filemaybe_redirect_or_append_path {
() => {
// Module: crate::services::fs::serve_dir::open_file
// Provides: {"maybe_redirect_or_append_path"}
// Dependencies: {}
async fn maybe_redirect_or_append_path (path_to_file : & mut PathBuf , uri : & Uri , append_index_html_on_directories : bool ,) -> Option < OpenFileOutput > { if ! is_dir (path_to_file) . await { return None ; } if ! append_index_html_on_directories { return Some (OpenFileOutput :: FileNotFound) ; } if uri . path () . ends_with ('/') { path_to_file . push ("index.html") ; None } else { let uri = match append_slash_on_path (uri . clone ()) { Ok (uri) => uri , Err (err) => return Some (err) , } ; let location = HeaderValue :: from_str (& uri . to_string ()) . unwrap () ; Some (OpenFileOutput :: Redirect { location }) } }
};
}
