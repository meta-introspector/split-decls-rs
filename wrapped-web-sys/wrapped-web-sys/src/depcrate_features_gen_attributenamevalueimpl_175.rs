// Generated macro for impl_175 (impl)
macro_rules! Depcrate_features_gen_AttributeNameValueimpl_175 {
() => {
// Module: crate::features::gen_AttributeNameValue
// Provides: {"impl_175"}
// Dependencies: {}
impl AttributeNameValue { # [doc = "Construct a new `AttributeNameValue`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"] pub fn new (name : & str , value : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_value (value) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_value()` instead."] pub fn value (& mut self , val : & str) -> & mut Self { self . set_value (val) ; self } }
};
}
