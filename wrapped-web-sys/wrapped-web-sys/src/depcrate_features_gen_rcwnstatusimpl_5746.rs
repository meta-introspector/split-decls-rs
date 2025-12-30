// Generated macro for impl_5746 (impl)
macro_rules! Depcrate_features_gen_RcwnStatusimpl_5746 {
() => {
// Module: crate::features::gen_RcwnStatus
// Provides: {"impl_5746"}
// Dependencies: {}
impl RcwnStatus { # [doc = "Construct a new `RcwnStatus`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_cache_not_slow_count()` instead."] pub fn cache_not_slow_count (& mut self , val : u32) -> & mut Self { self . set_cache_not_slow_count (val) ; self } # [deprecated = "Use `set_cache_slow_count()` instead."] pub fn cache_slow_count (& mut self , val : u32) -> & mut Self { self . set_cache_slow_count (val) ; self } # [deprecated = "Use `set_perf_stats()` instead."] pub fn perf_stats (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_perf_stats (val) ; self } # [deprecated = "Use `set_rcwn_cache_won_count()` instead."] pub fn rcwn_cache_won_count (& mut self , val : u32) -> & mut Self { self . set_rcwn_cache_won_count (val) ; self } # [deprecated = "Use `set_rcwn_net_won_count()` instead."] pub fn rcwn_net_won_count (& mut self , val : u32) -> & mut Self { self . set_rcwn_net_won_count (val) ; self } # [deprecated = "Use `set_total_network_requests()` instead."] pub fn total_network_requests (& mut self , val : u32) -> & mut Self { self . set_total_network_requests (val) ; self } }
};
}
