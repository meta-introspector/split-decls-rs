// Generated macro for impl_1216 (impl)
macro_rules! Depcrate_services_fs_serve_dirimpl_1216 {
() => {
// Module: crate::services::fs::serve_dir
// Provides: {"impl_1216"}
// Dependencies: {}
impl ServeDir < DefaultServeDirFallback > { # [doc = " Create a new [`ServeDir`]."] pub fn new < P > (path : P) -> Self where P : AsRef < Path > , { let mut base = PathBuf :: from (".") ; base . push (path . as_ref ()) ; Self { base , buf_chunk_size : DEFAULT_CAPACITY , precompressed_variants : None , variant : ServeVariant :: Directory { append_index_html_on_directories : true , } , fallback : None , call_fallback_on_method_not_allowed : false , } } pub (crate) fn new_single_file < P > (path : P , mime : HeaderValue) -> Self where P : AsRef < Path > , { Self { base : path . as_ref () . to_owned () , buf_chunk_size : DEFAULT_CAPACITY , precompressed_variants : None , variant : ServeVariant :: SingleFile { mime } , fallback : None , call_fallback_on_method_not_allowed : false , } } }
};
}
