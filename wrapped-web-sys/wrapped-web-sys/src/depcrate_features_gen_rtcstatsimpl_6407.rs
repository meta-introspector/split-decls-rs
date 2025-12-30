// Generated macro for impl_6407 (impl)
macro_rules! Depcrate_features_gen_RtcStatsimpl_6407 {
() => {
// Module: crate::features::gen_RtcStats
// Provides: {"impl_6407"}
// Dependencies: {}
impl RtcStats { # [doc = "Construct a new `RtcStats`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcStats`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & str) -> & mut Self { self . set_id (val) ; self } # [deprecated = "Use `set_timestamp()` instead."] pub fn timestamp (& mut self , val : f64) -> & mut Self { self . set_timestamp (val) ; self } # [cfg (feature = "RtcStatsType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : RtcStatsType) -> & mut Self { self . set_type (val) ; self } }
};
}
