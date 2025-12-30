// Generated macro for impl_5635 (impl)
macro_rules! Depcrate_features_gen_PublicKeyCredentialUserEntityJsonimpl_5635 {
() => {
// Module: crate::features::gen_PublicKeyCredentialUserEntityJson
// Provides: {"impl_5635"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl PublicKeyCredentialUserEntityJson { # [doc = "Construct a new `PublicKeyCredentialUserEntityJson`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntityJson`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (display_name : & str , id : & str , name : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_display_name (display_name) ; ret . set_id (id) ; ret . set_name (name) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_display_name()` instead."] pub fn display_name (& mut self , val : & str) -> & mut Self { self . set_display_name (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & str) -> & mut Self { self . set_id (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } }
};
}
