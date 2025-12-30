// Generated macro for rw_to_scope (function)
macro_rules! Depcrate_cache_gcsrw_to_scope {
() => {
// Module: crate::cache::gcs
// Provides: {"rw_to_scope"}
// Dependencies: {}
fn rw_to_scope (mode : CacheMode) -> & 'static str { match mode { CacheMode :: ReadOnly => "https://www.googleapis.com/auth/devstorage.read_only" , CacheMode :: ReadWrite => "https://www.googleapis.com/auth/devstorage.read_write" , } }
};
}
