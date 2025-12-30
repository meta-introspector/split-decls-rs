// Generated macro for FileObjectSource (struct)
macro_rules! Depcrate_cache_cacheFileObjectSource {
() => {
// Module: crate::cache::cache
// Provides: {"FileObjectSource"}
// Dependencies: {}
# [doc = " Cache object sourced by a file."] # [derive (Clone)] pub struct FileObjectSource { # [doc = " Identifier for this object. Should be unique within a compilation unit."] # [doc = " Note that a compilation unit is a single source file in C/C++ and a crate in Rust."] pub key : String , # [doc = " Absolute path to the file."] pub path : PathBuf , # [doc = " Whether the file must be present on disk and is essential for the compilation."] pub optional : bool , }
};
}
