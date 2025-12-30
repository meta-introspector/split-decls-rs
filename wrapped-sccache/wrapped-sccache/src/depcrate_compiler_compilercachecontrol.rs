// Generated macro for CacheControl (enum)
macro_rules! Depcrate_compiler_compilerCacheControl {
() => {
// Module: crate::compiler::compiler
// Provides: {"CacheControl"}
// Dependencies: {}
# [doc = " Control of caching behavior."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum CacheControl { # [doc = " Default caching behavior."] Default , # [doc = " Do not cache the results of the compilation."] ForceNoCache , # [doc = " Ignore existing cache entries, force recompilation."] ForceRecache , }
};
}
