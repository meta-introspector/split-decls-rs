// Generated macro for impl_5840 (impl)
macro_rules! Depcrate_features_gen_RegisteredKeyimpl_5840 {
() => {
// Module: crate::features::gen_RegisteredKey
// Provides: {"impl_5840"}
// Dependencies: {}
impl RegisteredKey { # [doc = "Construct a new `RegisteredKey`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_app_id()` instead."] pub fn app_id (& mut self , val : Option < & str >) -> & mut Self { self . set_app_id (val) ; self } # [deprecated = "Use `set_key_handle()` instead."] pub fn key_handle (& mut self , val : & str) -> & mut Self { self . set_key_handle (val) ; self } # [deprecated = "Use `set_transports()` instead."] pub fn transports (& mut self , val : Option < & :: wasm_bindgen :: JsValue >) -> & mut Self { self . set_transports (val . unwrap_or (& :: wasm_bindgen :: JsValue :: NULL)) ; self } # [deprecated = "Use `set_version()` instead."] pub fn version (& mut self , val : & str) -> & mut Self { self . set_version (val) ; self } }
};
}
