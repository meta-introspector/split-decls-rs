// Generated macro for impl_24 (impl)
macro_rules! Depcrate_features_gen_AesCbcParamsimpl_24 {
() => {
// Module: crate::features::gen_AesCbcParams
// Provides: {"impl_24"}
// Dependencies: {}
impl AesCbcParams { # [doc = "Construct a new `AesCbcParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"] pub fn new (name : & str , iv : & :: js_sys :: Object) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_iv (iv) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_iv()` instead."] pub fn iv (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_iv (val) ; self } }
};
}
