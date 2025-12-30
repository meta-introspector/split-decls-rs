// Generated macro for impl_6002 (impl)
macro_rules! Depcrate_features_gen_RtcCodecStatsimpl_6002 {
() => {
// Module: crate::features::gen_RtcCodecStats
// Provides: {"impl_6002"}
// Dependencies: {}
impl RtcCodecStats { # [doc = "Construct a new `RtcCodecStats`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & str) -> & mut Self { self . set_id (val) ; self } # [deprecated = "Use `set_timestamp()` instead."] pub fn timestamp (& mut self , val : f64) -> & mut Self { self . set_timestamp (val) ; self } # [cfg (feature = "RtcStatsType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : RtcStatsType) -> & mut Self { self . set_type (val) ; self } # [deprecated = "Use `set_channels()` instead."] pub fn channels (& mut self , val : u32) -> & mut Self { self . set_channels (val) ; self } # [deprecated = "Use `set_clock_rate()` instead."] pub fn clock_rate (& mut self , val : u32) -> & mut Self { self . set_clock_rate (val) ; self } # [deprecated = "Use `set_codec()` instead."] pub fn codec (& mut self , val : & str) -> & mut Self { self . set_codec (val) ; self } # [deprecated = "Use `set_parameters()` instead."] pub fn parameters (& mut self , val : & str) -> & mut Self { self . set_parameters (val) ; self } # [deprecated = "Use `set_payload_type()` instead."] pub fn payload_type (& mut self , val : u32) -> & mut Self { self . set_payload_type (val) ; self } }
};
}
