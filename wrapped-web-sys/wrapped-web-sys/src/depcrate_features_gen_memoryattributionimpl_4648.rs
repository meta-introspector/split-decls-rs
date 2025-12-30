// Generated macro for impl_4648 (impl)
macro_rules! Depcrate_features_gen_MemoryAttributionimpl_4648 {
() => {
// Module: crate::features::gen_MemoryAttribution
// Provides: {"impl_4648"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl MemoryAttribution { # [doc = "Construct a new `MemoryAttribution`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MemoryAttribution`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "MemoryAttributionContainer")] # [deprecated = "Use `set_container()` instead."] pub fn container (& mut self , val : & MemoryAttributionContainer) -> & mut Self { self . set_container (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_scope()` instead."] pub fn scope (& mut self , val : & str) -> & mut Self { self . set_scope (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_url()` instead."] pub fn url (& mut self , val : & str) -> & mut Self { self . set_url (val) ; self } }
};
}
