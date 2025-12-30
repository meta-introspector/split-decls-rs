// Generated macro for impl_6478 (impl)
macro_rules! Depcrate_features_gen_RtcrtpContributingSourceStatsimpl_6478 {
() => {
// Module: crate::features::gen_RtcrtpContributingSourceStats
// Provides: {"impl_6478"}
// Dependencies: {}
impl RtcrtpContributingSourceStats { # [doc = "Construct a new `RtcrtpContributingSourceStats`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & str) -> & mut Self { self . set_id (val) ; self } # [deprecated = "Use `set_timestamp()` instead."] pub fn timestamp (& mut self , val : f64) -> & mut Self { self . set_timestamp (val) ; self } # [cfg (feature = "RtcStatsType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : RtcStatsType) -> & mut Self { self . set_type (val) ; self } # [deprecated = "Use `set_contributor_ssrc()` instead."] pub fn contributor_ssrc (& mut self , val : u32) -> & mut Self { self . set_contributor_ssrc (val) ; self } # [deprecated = "Use `set_inbound_rtp_stream_id()` instead."] pub fn inbound_rtp_stream_id (& mut self , val : & str) -> & mut Self { self . set_inbound_rtp_stream_id (val) ; self } }
};
}
