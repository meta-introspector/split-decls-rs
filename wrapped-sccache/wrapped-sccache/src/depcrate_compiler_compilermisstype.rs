// Generated macro for MissType (enum)
macro_rules! Depcrate_compiler_compilerMissType {
() => {
// Module: crate::compiler::compiler
// Provides: {"MissType"}
// Dependencies: {}
# [doc = " Specifics about cache misses."] # [derive (Debug , PartialEq , Eq)] pub enum MissType { # [doc = " The compilation was not found in the cache, nothing more."] Normal , # [doc = " Do not cache the results of the compilation."] ForcedNoCache , # [doc = " Cache lookup was overridden, recompilation was forced."] ForcedRecache , # [doc = " Cache took too long to respond."] TimedOut , # [doc = " Error reading from cache"] CacheReadError , }
};
}
