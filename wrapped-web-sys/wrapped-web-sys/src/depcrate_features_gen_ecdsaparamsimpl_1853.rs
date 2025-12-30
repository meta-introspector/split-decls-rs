// Generated macro for impl_1853 (impl)
macro_rules! Depcrate_features_gen_EcdsaParamsimpl_1853 {
() => {
// Module: crate::features::gen_EcdsaParams
// Provides: {"impl_1853"}
// Dependencies: {}
impl EcdsaParams { # [doc = "Construct a new `EcdsaParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `EcdsaParams`*"] pub fn new (name : & str , hash : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_hash (hash) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_hash()` instead."] pub fn hash (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_hash (val) ; self } }
};
}
