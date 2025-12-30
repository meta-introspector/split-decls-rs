// Generated macro for impl_3865 (impl)
macro_rules! Depcrate_features_gen_IdbObjectStoreParametersimpl_3865 {
() => {
// Module: crate::features::gen_IdbObjectStoreParameters
// Provides: {"impl_3865"}
// Dependencies: {}
impl IdbObjectStoreParameters { # [doc = "Construct a new `IdbObjectStoreParameters`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_auto_increment()` instead."] pub fn auto_increment (& mut self , val : bool) -> & mut Self { self . set_auto_increment (val) ; self } # [deprecated = "Use `set_key_path()` instead."] pub fn key_path (& mut self , val : Option < & :: wasm_bindgen :: JsValue >) -> & mut Self { self . set_key_path (val . unwrap_or (& :: wasm_bindgen :: JsValue :: NULL)) ; self } }
};
}
