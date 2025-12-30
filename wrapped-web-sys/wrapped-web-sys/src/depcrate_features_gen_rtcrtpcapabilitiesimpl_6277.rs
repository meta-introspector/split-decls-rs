// Generated macro for impl_6277 (impl)
macro_rules! Depcrate_features_gen_RtcRtpCapabilitiesimpl_6277 {
() => {
// Module: crate::features::gen_RtcRtpCapabilities
// Provides: {"impl_6277"}
// Dependencies: {}
impl RtcRtpCapabilities { # [doc = "Construct a new `RtcRtpCapabilities`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"] pub fn new (codecs : & :: wasm_bindgen :: JsValue , header_extensions : & :: wasm_bindgen :: JsValue ,) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_codecs (codecs) ; ret . set_header_extensions (header_extensions) ; ret } # [deprecated = "Use `set_codecs()` instead."] pub fn codecs (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_codecs (val) ; self } # [deprecated = "Use `set_header_extensions()` instead."] pub fn header_extensions (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_header_extensions (val) ; self } }
};
}
