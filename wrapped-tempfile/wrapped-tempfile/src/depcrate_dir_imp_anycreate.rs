// Generated macro for create (function)
macro_rules! Depcrate_dir_imp_anycreate {
() => {
// Module: crate::dir::imp::any
// Provides: {"create"}
// Dependencies: {}
pub fn create (path : PathBuf , permissions : Option < & std :: fs :: Permissions > , disable_cleanup : bool ,) -> io :: Result < TempDir > { if permissions . map_or (false , | p | p . readonly ()) { return not_supported ("changing permissions is not supported on this platform") ; } fs :: create_dir (& path) . with_err_path (| | & path) . map (| _ | TempDir { path : path . into_boxed_path () , disable_cleanup , }) }
};
}
