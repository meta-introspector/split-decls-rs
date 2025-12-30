// Generated macro for impl_6125 (impl)
macro_rules! Depcrate_features_gen_RtcIceComponentStatsimpl_6125 {
() => {
// Module: crate::features::gen_RtcIceComponentStats
// Provides: {"impl_6125"}
// Dependencies: {}
impl RtcIceComponentStats { # [doc = "Construct a new `RtcIceComponentStats`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & str) -> & mut Self { self . set_id (val) ; self } # [deprecated = "Use `set_timestamp()` instead."] pub fn timestamp (& mut self , val : f64) -> & mut Self { self . set_timestamp (val) ; self } # [cfg (feature = "RtcStatsType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : RtcStatsType) -> & mut Self { self . set_type (val) ; self } # [deprecated = "Use `set_active_connection()` instead."] pub fn active_connection (& mut self , val : bool) -> & mut Self { self . set_active_connection (val) ; self } # [deprecated = "Use `set_bytes_received()` instead."] pub fn bytes_received (& mut self , val : u32) -> & mut Self { self . set_bytes_received (val) ; self } # [deprecated = "Use `set_bytes_sent()` instead."] pub fn bytes_sent (& mut self , val : u32) -> & mut Self { self . set_bytes_sent (val) ; self } # [deprecated = "Use `set_component()` instead."] pub fn component (& mut self , val : i32) -> & mut Self { self . set_component (val) ; self } # [deprecated = "Use `set_transport_id()` instead."] pub fn transport_id (& mut self , val : & str) -> & mut Self { self . set_transport_id (val) ; self } }
};
}
