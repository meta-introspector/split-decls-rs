// Generated macro for RlibDepReader (struct)
macro_rules! Depcrate_compiler_rustRlibDepReader {
() => {
// Module: crate::compiler::rust
// Provides: {"RlibDepReader"}
// Dependencies: {}
# [cfg (feature = "dist-client")] # [derive (Debug)] struct RlibDepReader { cache : Mutex < LruCache < PathBuf , RlibDepsDetail , RandomState , DepsSize > > , executable : PathBuf , ls_arg : String , }
};
}
