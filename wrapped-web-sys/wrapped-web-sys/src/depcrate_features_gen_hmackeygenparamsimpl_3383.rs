// Generated macro for impl_3383 (impl)
macro_rules! Depcrate_features_gen_HmacKeyGenParamsimpl_3383 {
() => {
// Module: crate::features::gen_HmacKeyGenParams
// Provides: {"impl_3383"}
// Dependencies: {}
impl HmacKeyGenParams { # [doc = "Construct a new `HmacKeyGenParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `HmacKeyGenParams`*"] pub fn new (name : & str , hash : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_hash (hash) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_hash()` instead."] pub fn hash (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_hash (val) ; self } # [deprecated = "Use `set_length()` instead."] pub fn length (& mut self , val : u32) -> & mut Self { self . set_length (val) ; self } }
};
}
