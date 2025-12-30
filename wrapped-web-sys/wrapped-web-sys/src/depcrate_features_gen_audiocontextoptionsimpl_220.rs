// Generated macro for impl_220 (impl)
macro_rules! Depcrate_features_gen_AudioContextOptionsimpl_220 {
() => {
// Module: crate::features::gen_AudioContextOptions
// Provides: {"impl_220"}
// Dependencies: {}
impl AudioContextOptions { # [doc = "Construct a new `AudioContextOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_latency_hint()` instead."] pub fn latency_hint (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_latency_hint (val) ; self } # [deprecated = "Use `set_sample_rate()` instead."] pub fn sample_rate (& mut self , val : f32) -> & mut Self { self . set_sample_rate (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_sink_id()` instead."] pub fn sink_id (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_sink_id (val) ; self } }
};
}
