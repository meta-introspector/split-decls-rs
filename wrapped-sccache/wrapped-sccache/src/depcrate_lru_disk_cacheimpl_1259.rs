// Generated macro for impl_1259 (impl)
macro_rules! Depcrate_lru_disk_cacheimpl_1259 {
() => {
// Module: crate::lru_disk_cache
// Provides: {"impl_1259"}
// Dependencies: {}
impl StdError for Error { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { Error :: FileTooLarge => None , Error :: FileNotInCache => None , Error :: Io (e) => Some (e) , } } }
};
}
