// Generated macro for impl_3359 (impl)
macro_rules! Depcrate_features_gen_HkdfParamsimpl_3359 {
() => {
// Module: crate::features::gen_HkdfParams
// Provides: {"impl_3359"}
// Dependencies: {}
impl HkdfParams { # [doc = "Construct a new `HkdfParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `HkdfParams`*"] pub fn new (name : & str , hash : & :: wasm_bindgen :: JsValue , info : & :: js_sys :: Object , salt : & :: js_sys :: Object ,) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_hash (hash) ; ret . set_info (info) ; ret . set_salt (salt) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_hash()` instead."] pub fn hash (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_hash (val) ; self } # [deprecated = "Use `set_info()` instead."] pub fn info (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_info (val) ; self } # [deprecated = "Use `set_salt()` instead."] pub fn salt (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_salt (val) ; self } }
};
}
