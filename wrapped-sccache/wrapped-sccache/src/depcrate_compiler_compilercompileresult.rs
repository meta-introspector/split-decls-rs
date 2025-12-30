// Generated macro for CompileResult (enum)
macro_rules! Depcrate_compiler_compilerCompileResult {
() => {
// Module: crate::compiler::compiler
// Provides: {"CompileResult"}
// Dependencies: {}
# [doc = " The result of a compilation or cache retrieval."] pub enum CompileResult { # [doc = " An error made the compilation not possible."] Error , # [doc = " Result was found in cache."] CacheHit (Duration) , # [doc = " Result was not found in cache."] # [doc = ""] # [doc = " The `CacheWriteFuture` will resolve when the result is finished"] # [doc = " being stored in the cache."] CacheMiss (MissType , DistType , Duration , Pin < Box < dyn Future < Output = Result < CacheWriteInfo > > + Send > > ,) , # [doc = " Not in cache and do not cache the results of the compilation."] NotCached (DistType , Duration) , # [doc = " Not in cache, but the compilation result was determined to be not cacheable."] NotCacheable (DistType , Duration) , # [doc = " Not in cache, but compilation failed."] CompileFailed (DistType , Duration) , }
};
}
