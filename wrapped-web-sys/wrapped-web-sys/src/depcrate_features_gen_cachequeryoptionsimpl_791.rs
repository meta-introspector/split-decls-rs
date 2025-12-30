// Generated macro for impl_791 (impl)
macro_rules! Depcrate_features_gen_CacheQueryOptionsimpl_791 {
() => {
// Module: crate::features::gen_CacheQueryOptions
// Provides: {"impl_791"}
// Dependencies: {}
impl CacheQueryOptions { # [doc = "Construct a new `CacheQueryOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_cache_name()` instead."] pub fn cache_name (& mut self , val : & str) -> & mut Self { self . set_cache_name (val) ; self } # [deprecated = "Use `set_ignore_method()` instead."] pub fn ignore_method (& mut self , val : bool) -> & mut Self { self . set_ignore_method (val) ; self } # [deprecated = "Use `set_ignore_search()` instead."] pub fn ignore_search (& mut self , val : bool) -> & mut Self { self . set_ignore_search (val) ; self } # [deprecated = "Use `set_ignore_vary()` instead."] pub fn ignore_vary (& mut self , val : bool) -> & mut Self { self . set_ignore_vary (val) ; self } }
};
}
