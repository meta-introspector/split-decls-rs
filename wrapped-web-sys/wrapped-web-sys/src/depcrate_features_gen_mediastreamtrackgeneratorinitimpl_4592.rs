// Generated macro for impl_4592 (impl)
macro_rules! Depcrate_features_gen_MediaStreamTrackGeneratorInitimpl_4592 {
() => {
// Module: crate::features::gen_MediaStreamTrackGeneratorInit
// Provides: {"impl_4592"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl MediaStreamTrackGeneratorInit { # [doc = "Construct a new `MediaStreamTrackGeneratorInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaStreamTrackGeneratorInit`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (kind : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_kind (kind) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_kind()` instead."] pub fn kind (& mut self , val : & str) -> & mut Self { self . set_kind (val) ; self } }
};
}
