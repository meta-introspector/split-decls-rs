// Generated macro for impl_4547 (impl)
macro_rules! Depcrate_features_gen_MediaStreamConstraintsimpl_4547 {
() => {
// Module: crate::features::gen_MediaStreamConstraints
// Provides: {"impl_4547"}
// Dependencies: {}
impl MediaStreamConstraints { # [doc = "Construct a new `MediaStreamConstraints`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_audio()` instead."] pub fn audio (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_audio (val) ; self } # [deprecated = "Use `set_fake()` instead."] pub fn fake (& mut self , val : bool) -> & mut Self { self . set_fake (val) ; self } # [deprecated = "Use `set_peer_identity()` instead."] pub fn peer_identity (& mut self , val : Option < & str >) -> & mut Self { self . set_peer_identity (val) ; self } # [deprecated = "Use `set_picture()` instead."] pub fn picture (& mut self , val : bool) -> & mut Self { self . set_picture (val) ; self } # [deprecated = "Use `set_video()` instead."] pub fn video (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_video (val) ; self } }
};
}
