// Generated macro for impl_4131 (impl)
macro_rules! Depcrate_features_gen_KeyIdsInitDataimpl_4131 {
() => {
// Module: crate::features::gen_KeyIdsInitData
// Provides: {"impl_4131"}
// Dependencies: {}
impl KeyIdsInitData { # [doc = "Construct a new `KeyIdsInitData`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `KeyIdsInitData`*"] pub fn new (kids : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_kids (kids) ; ret } # [deprecated = "Use `set_kids()` instead."] pub fn kids (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_kids (val) ; self } }
};
}
