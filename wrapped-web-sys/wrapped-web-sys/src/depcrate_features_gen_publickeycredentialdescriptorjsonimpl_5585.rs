// Generated macro for impl_5585 (impl)
macro_rules! Depcrate_features_gen_PublicKeyCredentialDescriptorJsonimpl_5585 {
() => {
// Module: crate::features::gen_PublicKeyCredentialDescriptorJson
// Provides: {"impl_5585"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl PublicKeyCredentialDescriptorJson { # [doc = "Construct a new `PublicKeyCredentialDescriptorJson`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (id : & str , type_ : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_id (id) ; ret . set_type (type_) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & str) -> & mut Self { self . set_id (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_transports()` instead."] pub fn transports (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_transports (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : & str) -> & mut Self { self . set_type (val) ; self } }
};
}
