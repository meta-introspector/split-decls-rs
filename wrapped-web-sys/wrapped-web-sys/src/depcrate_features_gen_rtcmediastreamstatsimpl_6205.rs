// Generated macro for impl_6205 (impl)
macro_rules! Depcrate_features_gen_RtcMediaStreamStatsimpl_6205 {
() => {
// Module: crate::features::gen_RtcMediaStreamStats
// Provides: {"impl_6205"}
// Dependencies: {}
impl RtcMediaStreamStats { # [doc = "Construct a new `RtcMediaStreamStats`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & str) -> & mut Self { self . set_id (val) ; self } # [deprecated = "Use `set_timestamp()` instead."] pub fn timestamp (& mut self , val : f64) -> & mut Self { self . set_timestamp (val) ; self } # [cfg (feature = "RtcStatsType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : RtcStatsType) -> & mut Self { self . set_type (val) ; self } # [deprecated = "Use `set_stream_identifier()` instead."] pub fn stream_identifier (& mut self , val : & str) -> & mut Self { self . set_stream_identifier (val) ; self } # [deprecated = "Use `set_track_ids()` instead."] pub fn track_ids (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_track_ids (val) ; self } }
};
}
