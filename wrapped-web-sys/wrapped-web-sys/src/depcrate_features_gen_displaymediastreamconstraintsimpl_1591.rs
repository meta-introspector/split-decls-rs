// Generated macro for impl_1591 (impl)
macro_rules! Depcrate_features_gen_DisplayMediaStreamConstraintsimpl_1591 {
() => {
// Module: crate::features::gen_DisplayMediaStreamConstraints
// Provides: {"impl_1591"}
// Dependencies: {}
impl DisplayMediaStreamConstraints { # [doc = "Construct a new `DisplayMediaStreamConstraints`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_audio()` instead."] pub fn audio (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_audio (val) ; self } # [deprecated = "Use `set_video()` instead."] pub fn video (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_video (val) ; self } }
};
}
