// Generated macro for impl_42 (impl)
macro_rules! Depcrate_features_gen_AesGcmParamsimpl_42 {
() => {
// Module: crate::features::gen_AesGcmParams
// Provides: {"impl_42"}
// Dependencies: {}
impl AesGcmParams { # [doc = "Construct a new `AesGcmParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"] pub fn new (name : & str , iv : & :: js_sys :: Object) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_iv (iv) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_additional_data()` instead."] pub fn additional_data (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_additional_data (val) ; self } # [deprecated = "Use `set_iv()` instead."] pub fn iv (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_iv (val) ; self } # [deprecated = "Use `set_tag_length()` instead."] pub fn tag_length (& mut self , val : u8) -> & mut Self { self . set_tag_length (val) ; self } }
};
}
