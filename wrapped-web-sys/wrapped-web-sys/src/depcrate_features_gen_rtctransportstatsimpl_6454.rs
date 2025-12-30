// Generated macro for impl_6454 (impl)
macro_rules! Depcrate_features_gen_RtcTransportStatsimpl_6454 {
() => {
// Module: crate::features::gen_RtcTransportStats
// Provides: {"impl_6454"}
// Dependencies: {}
impl RtcTransportStats { # [doc = "Construct a new `RtcTransportStats`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & str) -> & mut Self { self . set_id (val) ; self } # [deprecated = "Use `set_timestamp()` instead."] pub fn timestamp (& mut self , val : f64) -> & mut Self { self . set_timestamp (val) ; self } # [cfg (feature = "RtcStatsType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : RtcStatsType) -> & mut Self { self . set_type (val) ; self } # [deprecated = "Use `set_bytes_received()` instead."] pub fn bytes_received (& mut self , val : u32) -> & mut Self { self . set_bytes_received (val) ; self } # [deprecated = "Use `set_bytes_sent()` instead."] pub fn bytes_sent (& mut self , val : u32) -> & mut Self { self . set_bytes_sent (val) ; self } }
};
}
