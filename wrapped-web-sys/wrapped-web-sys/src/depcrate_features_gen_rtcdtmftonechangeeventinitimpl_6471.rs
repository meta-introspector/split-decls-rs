// Generated macro for impl_6471 (impl)
macro_rules! Depcrate_features_gen_RtcdtmfToneChangeEventInitimpl_6471 {
() => {
// Module: crate::features::gen_RtcdtmfToneChangeEventInit
// Provides: {"impl_6471"}
// Dependencies: {}
impl RtcdtmfToneChangeEventInit { # [doc = "Construct a new `RtcdtmfToneChangeEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_tone()` instead."] pub fn tone (& mut self , val : & str) -> & mut Self { self . set_tone (val) ; self } }
};
}
