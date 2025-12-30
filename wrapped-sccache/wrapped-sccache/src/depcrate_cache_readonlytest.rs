// Generated macro for test (module)
macro_rules! Depcrate_cache_readonlytest {
() => {
// Module: crate::cache::readonly
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use futures :: FutureExt ; use super :: * ; use crate :: test :: mock_storage :: MockStorage ; # [test] fn readonly_storage_is_readonly () { let storage = ReadOnlyStorage (Arc :: new (MockStorage :: new (None , false))) ; assert_eq ! (storage . check () . now_or_never () . unwrap () . unwrap () , CacheMode :: ReadOnly) ; } # [test] fn readonly_storage_forwards_preprocessor_cache_mode_config () { let storage_no_preprocessor_cache = ReadOnlyStorage (Arc :: new (MockStorage :: new (None , false))) ; assert ! (! storage_no_preprocessor_cache . preprocessor_cache_mode_config () . use_preprocessor_cache_mode) ; let storage_with_preprocessor_cache = ReadOnlyStorage (Arc :: new (MockStorage :: new (None , true))) ; assert ! (storage_with_preprocessor_cache . preprocessor_cache_mode_config () . use_preprocessor_cache_mode) ; } # [test] fn readonly_storage_put_err () { let runtime = tokio :: runtime :: Builder :: new_current_thread () . enable_all () . worker_threads (1) . build () . unwrap () ; let storage = ReadOnlyStorage (Arc :: new (MockStorage :: new (None , true))) ; runtime . block_on (async move { assert_eq ! (storage . put ("test1" , CacheWrite :: default ()) . await . unwrap_err () . to_string () , "Cannot write to read-only storage") ; assert_eq ! (storage . put_preprocessor_cache_entry ("test1" , PreprocessorCacheEntry :: default ()) . await . unwrap_err () . to_string () , "Cannot write to read-only storage") ; }) ; } }
};
}
