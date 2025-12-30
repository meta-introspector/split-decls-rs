// Generated macro for impl_1258 (impl)
macro_rules! Depcrate_lru_disk_cacheimpl_1258 {
() => {
// Module: crate::lru_disk_cache
// Provides: {"impl_1258"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: FileTooLarge => write ! (f , "File too large") , Error :: FileNotInCache => write ! (f , "File not in cache") , Error :: Io (e) => write ! (f , "{}" , e) , } } }
};
}
