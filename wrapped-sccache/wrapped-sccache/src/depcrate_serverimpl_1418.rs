// Generated macro for impl_1418 (impl)
macro_rules! Depcrate_serverimpl_1418 {
() => {
// Module: crate::server
// Provides: {"impl_1418"}
// Dependencies: {}
impl Default for ServerStats { fn default () -> ServerStats { ServerStats { compile_requests : u64 :: default () , requests_unsupported_compiler : u64 :: default () , requests_not_compile : u64 :: default () , requests_not_cacheable : u64 :: default () , requests_executed : u64 :: default () , cache_errors : PerLanguageCount :: new () , cache_hits : PerLanguageCount :: new () , cache_misses : PerLanguageCount :: new () , cache_timeouts : u64 :: default () , cache_read_errors : u64 :: default () , non_cacheable_compilations : u64 :: default () , forced_recaches : u64 :: default () , cache_write_errors : u64 :: default () , cache_writes : u64 :: default () , cache_write_duration : Duration :: new (0 , 0) , cache_read_hit_duration : Duration :: new (0 , 0) , compilations : u64 :: default () , compiler_write_duration : Duration :: new (0 , 0) , compile_fails : u64 :: default () , not_cached : HashMap :: new () , dist_compiles : HashMap :: new () , dist_errors : u64 :: default () , } } }
};
}
