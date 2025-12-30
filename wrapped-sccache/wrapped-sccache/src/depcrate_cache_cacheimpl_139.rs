// Generated macro for impl_139 (impl)
macro_rules! Depcrate_cache_cacheimpl_139 {
() => {
// Module: crate::cache::cache
// Provides: {"impl_139"}
// Dependencies: {}
impl fmt :: Debug for Cache { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Cache :: Hit (_) => write ! (f , "Cache::Hit(...)") , Cache :: Miss => write ! (f , "Cache::Miss") , Cache :: None => write ! (f , "Cache::None") , Cache :: Recache => write ! (f , "Cache::Recache") , } } }
};
}
