// Generated macro for impl_577 (impl)
macro_rules! Depcrate_compiler_compilerimpl_577 {
() => {
// Module: crate::compiler::compiler
// Provides: {"impl_577"}
// Dependencies: {}
# [doc = " Can't derive(Debug) because of `CacheWriteFuture`."] impl fmt :: Debug for CompileResult { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { CompileResult :: Error => write ! (f , "CompileResult::Error") , CompileResult :: CacheHit (ref d) => write ! (f , "CompileResult::CacheHit({:?})" , d) , CompileResult :: CacheMiss (ref m , ref dt , ref d , _) => { write ! (f , "CompileResult::CacheMiss({:?}, {:?}, {:?}, _)" , d , m , dt) } CompileResult :: NotCached (ref dt , ref d) => { write ! (f , "CompileResult::NotCached({:?}, {:?}_" , dt , d) } CompileResult :: NotCacheable (ref dt , ref d) => { write ! (f , "CompileResult::NotCacheable({:?}, {:?}_" , dt , d) } CompileResult :: CompileFailed (ref dt , ref d) => { write ! (f , "CompileResult::CompileFailed({:?}, {:?})" , dt , d) } } } }
};
}
