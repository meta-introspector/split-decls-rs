// Generated macro for impl_6105 (impl)
macro_rules! Depcrate_features_gen_RtcIceCandidateInitimpl_6105 {
() => {
// Module: crate::features::gen_RtcIceCandidateInit
// Provides: {"impl_6105"}
// Dependencies: {}
impl RtcIceCandidateInit { # [doc = "Construct a new `RtcIceCandidateInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"] pub fn new (candidate : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_candidate (candidate) ; ret } # [deprecated = "Use `set_candidate()` instead."] pub fn candidate (& mut self , val : & str) -> & mut Self { self . set_candidate (val) ; self } # [deprecated = "Use `set_sdp_m_line_index()` instead."] pub fn sdp_m_line_index (& mut self , val : Option < u16 >) -> & mut Self { self . set_sdp_m_line_index (val) ; self } # [deprecated = "Use `set_sdp_mid()` instead."] pub fn sdp_mid (& mut self , val : Option < & str >) -> & mut Self { self . set_sdp_mid (val) ; self } }
};
}
