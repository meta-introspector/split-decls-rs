// Generated macro for create_named (function)
macro_rules! Depcrate_filecreate_named {
() => {
// Module: crate::file
// Provides: {"create_named"}
// Dependencies: {}
pub (crate) fn create_named (path : PathBuf , open_options : & mut OpenOptions , permissions : Option < & std :: fs :: Permissions > , keep : bool ,) -> io :: Result < NamedTempFile > { imp :: create_named (& path , open_options , permissions) . with_err_path (| | path . clone ()) . map (| file | NamedTempFile { path : TempPath { path : path . into_boxed_path () , disable_cleanup : keep , } , file , }) }
};
}
