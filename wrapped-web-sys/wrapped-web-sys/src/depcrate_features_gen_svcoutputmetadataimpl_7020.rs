// Generated macro for impl_7020 (impl)
macro_rules! Depcrate_features_gen_SvcOutputMetadataimpl_7020 {
() => {
// Module: crate::features::gen_SvcOutputMetadata
// Provides: {"impl_7020"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl SvcOutputMetadata { # [doc = "Construct a new `SvcOutputMetadata`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `SvcOutputMetadata`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_temporal_layer_id()` instead."] pub fn temporal_layer_id (& mut self , val : u32) -> & mut Self { self . set_temporal_layer_id (val) ; self } }
};
}
