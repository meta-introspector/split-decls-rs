// Generated macro for impl_3371 (impl)
macro_rules! Depcrate_features_gen_HmacImportParamsimpl_3371 {
() => {
// Module: crate::features::gen_HmacImportParams
// Provides: {"impl_3371"}
// Dependencies: {}
impl HmacImportParams { # [doc = "Construct a new `HmacImportParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `HmacImportParams`*"] pub fn new (name : & str , hash : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_hash (hash) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_hash()` instead."] pub fn hash (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_hash (val) ; self } }
};
}
