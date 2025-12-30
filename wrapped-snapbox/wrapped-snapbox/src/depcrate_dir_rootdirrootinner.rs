// Generated macro for DirRootInner (enum)
macro_rules! Depcrate_dir_rootDirRootInner {
() => {
// Module: crate::dir::root
// Provides: {"DirRootInner"}
// Dependencies: {}
# [derive (Debug)] enum DirRootInner { None , Immutable (std :: path :: PathBuf) , # [cfg (feature = "dir")] MutablePath (std :: path :: PathBuf) , # [cfg (feature = "dir")] MutableTemp { temp : tempfile :: TempDir , path : std :: path :: PathBuf , } , }
};
}
