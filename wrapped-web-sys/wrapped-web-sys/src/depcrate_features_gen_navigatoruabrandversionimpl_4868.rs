// Generated macro for impl_4868 (impl)
macro_rules! Depcrate_features_gen_NavigatorUaBrandVersionimpl_4868 {
() => {
// Module: crate::features::gen_NavigatorUaBrandVersion
// Provides: {"impl_4868"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl NavigatorUaBrandVersion { # [doc = "Construct a new `NavigatorUaBrandVersion`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `NavigatorUaBrandVersion`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_brand()` instead."] pub fn brand (& mut self , val : & str) -> & mut Self { self . set_brand (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_version()` instead."] pub fn version (& mut self , val : & str) -> & mut Self { self . set_version (val) ; self } }
};
}
