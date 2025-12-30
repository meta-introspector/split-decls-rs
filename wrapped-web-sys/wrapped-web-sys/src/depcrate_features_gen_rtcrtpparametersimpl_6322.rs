// Generated macro for impl_6322 (impl)
macro_rules! Depcrate_features_gen_RtcRtpParametersimpl_6322 {
() => {
// Module: crate::features::gen_RtcRtpParameters
// Provides: {"impl_6322"}
// Dependencies: {}
impl RtcRtpParameters { # [doc = "Construct a new `RtcRtpParameters`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_codecs()` instead."] pub fn codecs (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_codecs (val) ; self } # [deprecated = "Use `set_encodings()` instead."] pub fn encodings (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_encodings (val) ; self } # [deprecated = "Use `set_header_extensions()` instead."] pub fn header_extensions (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_header_extensions (val) ; self } # [cfg (feature = "RtcRtcpParameters")] # [deprecated = "Use `set_rtcp()` instead."] pub fn rtcp (& mut self , val : & RtcRtcpParameters) -> & mut Self { self . set_rtcp (val) ; self } }
};
}
