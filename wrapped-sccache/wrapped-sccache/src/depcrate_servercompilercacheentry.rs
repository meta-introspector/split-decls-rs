// Generated macro for CompilerCacheEntry (struct)
macro_rules! Depcrate_serverCompilerCacheEntry {
() => {
// Module: crate::server
// Provides: {"CompilerCacheEntry"}
// Dependencies: {}
# [doc = " entry of the compiler cache"] struct CompilerCacheEntry < C > { # [doc = " compiler argument trait obj"] pub compiler : Box < dyn Compiler < C > > , # [doc = " modification time of the compilers executable file"] pub mtime : FileTime , # [doc = " distributed compilation extra info"] pub dist_info : Option < (PathBuf , FileTime) > , }
};
}
