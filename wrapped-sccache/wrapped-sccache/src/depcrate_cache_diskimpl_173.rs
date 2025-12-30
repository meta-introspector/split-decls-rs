// Generated macro for impl_173 (impl)
macro_rules! Depcrate_cache_diskimpl_173 {
() => {
// Module: crate::cache::disk
// Provides: {"impl_173"}
// Dependencies: {}
impl LazyDiskCache { fn get_or_init (& mut self) -> Result < & mut LruDiskCache > { match self { LazyDiskCache :: Uninit { root , max_size } => { * self = LazyDiskCache :: Init (LruDiskCache :: new (& root , * max_size) ?) ; self . get_or_init () } LazyDiskCache :: Init (d) => Ok (d) , } } fn get (& mut self) -> Option < & mut LruDiskCache > { match self { LazyDiskCache :: Uninit { .. } => None , LazyDiskCache :: Init (d) => Some (d) , } } fn capacity (& self) -> u64 { match self { LazyDiskCache :: Uninit { max_size , .. } => * max_size , LazyDiskCache :: Init (d) => d . capacity () , } } fn path (& self) -> & Path { match self { LazyDiskCache :: Uninit { root , .. } => root . as_ref () , LazyDiskCache :: Init (d) => d . path () , } } }
};
}
