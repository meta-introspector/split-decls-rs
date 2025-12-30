// Generated macro for impl_208 (impl)
macro_rules! Depcrate_dir_opsimpl_208 {
() => {
// Module: crate::dir::ops
// Provides: {"impl_208"}
// Dependencies: {}
# [cfg (feature = "dir")] impl Iterator for Walk { type Item = Result < std :: path :: PathBuf , std :: io :: Error > ; fn next (& mut self) -> Option < Self :: Item > { while let Some (entry) = self . inner . next () . map (| e | { e . map (walkdir :: DirEntry :: into_path) . map_err (std :: io :: Error :: from) }) { if entry . as_ref () . ok () . and_then (| e | e . file_name ()) != Some (std :: ffi :: OsStr :: new (".keep")) { return Some (entry) ; } } None } }
};
}
