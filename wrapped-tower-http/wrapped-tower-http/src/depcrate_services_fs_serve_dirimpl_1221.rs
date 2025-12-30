// Generated macro for impl_1221 (impl)
macro_rules! Depcrate_services_fs_serve_dirimpl_1221 {
() => {
// Module: crate::services::fs::serve_dir
// Provides: {"impl_1221"}
// Dependencies: {}
impl ServeVariant { fn build_and_validate_path (& self , base_path : & Path , requested_path : & str) -> Option < PathBuf > { match self { ServeVariant :: Directory { append_index_html_on_directories : _ , } => { let path = requested_path . trim_start_matches ('/') ; let path_decoded = percent_decode (path . as_ref ()) . decode_utf8 () . ok () ? ; let path_decoded = Path :: new (& * path_decoded) ; let mut path_to_file = base_path . to_path_buf () ; for component in path_decoded . components () { match component { Component :: Normal (comp) => { if Path :: new (& comp) . components () . all (| c | matches ! (c , Component :: Normal (_))) { path_to_file . push (comp) } else { return None ; } } Component :: CurDir => { } Component :: Prefix (_) | Component :: RootDir | Component :: ParentDir => { return None ; } } } Some (path_to_file) } ServeVariant :: SingleFile { mime : _ } => Some (base_path . to_path_buf ()) , } } }
};
}
