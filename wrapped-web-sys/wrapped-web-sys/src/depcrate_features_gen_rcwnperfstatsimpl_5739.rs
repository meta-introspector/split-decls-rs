// Generated macro for impl_5739 (impl)
macro_rules! Depcrate_features_gen_RcwnPerfStatsimpl_5739 {
() => {
// Module: crate::features::gen_RcwnPerfStats
// Provides: {"impl_5739"}
// Dependencies: {}
impl RcwnPerfStats { # [doc = "Construct a new `RcwnPerfStats`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_avg_long()` instead."] pub fn avg_long (& mut self , val : u32) -> & mut Self { self . set_avg_long (val) ; self } # [deprecated = "Use `set_avg_short()` instead."] pub fn avg_short (& mut self , val : u32) -> & mut Self { self . set_avg_short (val) ; self } # [deprecated = "Use `set_stddev_long()` instead."] pub fn stddev_long (& mut self , val : u32) -> & mut Self { self . set_stddev_long (val) ; self } }
};
}
