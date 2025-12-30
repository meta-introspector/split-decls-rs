// Generated macro for impl_175 (impl)
macro_rules! Depcrate_cache_diskimpl_175 {
() => {
// Module: crate::cache::disk
// Provides: {"impl_175"}
// Dependencies: {}
impl DiskCache { # [doc = " Create a new `DiskCache` rooted at `root`, with `max_size` as the maximum cache size on-disk, in bytes."] pub fn new < T : AsRef < OsStr > > (root : T , max_size : u64 , pool : & tokio :: runtime :: Handle , preprocessor_cache_mode_config : PreprocessorCacheModeConfig , rw_mode : CacheMode ,) -> DiskCache { DiskCache { lru : Arc :: new (Mutex :: new (LazyDiskCache :: Uninit { root : root . as_ref () . to_os_string () , max_size , })) , pool : pool . clone () , preprocessor_cache_mode_config , preprocessor_cache : Arc :: new (Mutex :: new (LazyDiskCache :: Uninit { root : Path :: new (root . as_ref ()) . join ("preprocessor") . into_os_string () , max_size , })) , rw_mode , } } }
};
}
