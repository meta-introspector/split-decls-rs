// Generated macro for impl_4655 (impl)
macro_rules! Depcrate_features_gen_MemoryAttributionContainerimpl_4655 {
() => {
// Module: crate::features::gen_MemoryAttributionContainer
// Provides: {"impl_4655"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl MemoryAttributionContainer { # [doc = "Construct a new `MemoryAttributionContainer`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MemoryAttributionContainer`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : & str) -> & mut Self { self . set_id (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_src()` instead."] pub fn src (& mut self , val : & str) -> & mut Self { self . set_src (val) ; self } }
};
}
