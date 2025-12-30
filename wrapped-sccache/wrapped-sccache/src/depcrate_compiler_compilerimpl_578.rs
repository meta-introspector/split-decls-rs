// Generated macro for impl_578 (impl)
macro_rules! Depcrate_compiler_compilerimpl_578 {
() => {
// Module: crate::compiler::compiler
// Provides: {"impl_578"}
// Dependencies: {}
# [doc = " Can't use derive(PartialEq) because of the `CacheWriteFuture`."] impl PartialEq < CompileResult > for CompileResult { fn eq (& self , other : & CompileResult) -> bool { match (self , other) { (& CompileResult :: Error , & CompileResult :: Error) => true , (& CompileResult :: CacheHit (_) , & CompileResult :: CacheHit (_)) => true , (CompileResult :: CacheMiss (m , dt , _ , _) , CompileResult :: CacheMiss (n , dt2 , _ , _)) => { m == n && dt == dt2 } (CompileResult :: NotCached (dt , _) , CompileResult :: NotCached (dt2 , _)) => dt == dt2 , (CompileResult :: NotCacheable (dt , _) , CompileResult :: NotCacheable (dt2 , _)) => dt == dt2 , (CompileResult :: CompileFailed (dt , _) , CompileResult :: CompileFailed (dt2 , _)) => { dt == dt2 } _ => false , } } }
};
}
