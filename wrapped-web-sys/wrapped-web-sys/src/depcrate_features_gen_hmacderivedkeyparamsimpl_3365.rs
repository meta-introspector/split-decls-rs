// Generated macro for impl_3365 (impl)
macro_rules! Depcrate_features_gen_HmacDerivedKeyParamsimpl_3365 {
() => {
// Module: crate::features::gen_HmacDerivedKeyParams
// Provides: {"impl_3365"}
// Dependencies: {}
impl HmacDerivedKeyParams { # [doc = "Construct a new `HmacDerivedKeyParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"] pub fn new (name : & str , hash : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_hash (hash) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_hash()` instead."] pub fn hash (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_hash (val) ; self } # [deprecated = "Use `set_length()` instead."] pub fn length (& mut self , val : u32) -> & mut Self { self . set_length (val) ; self } }
};
}
