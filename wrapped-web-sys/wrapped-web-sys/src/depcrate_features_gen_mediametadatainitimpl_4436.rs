// Generated macro for impl_4436 (impl)
macro_rules! Depcrate_features_gen_MediaMetadataInitimpl_4436 {
() => {
// Module: crate::features::gen_MediaMetadataInit
// Provides: {"impl_4436"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl MediaMetadataInit { # [doc = "Construct a new `MediaMetadataInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaMetadataInit`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_album()` instead."] pub fn album (& mut self , val : & str) -> & mut Self { self . set_album (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_artist()` instead."] pub fn artist (& mut self , val : & str) -> & mut Self { self . set_artist (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_artwork()` instead."] pub fn artwork (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_artwork (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_title()` instead."] pub fn title (& mut self , val : & str) -> & mut Self { self . set_title (val) ; self } }
};
}
