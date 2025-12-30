// Generated macro for impl_8435 (impl)
macro_rules! Depcrate_features_gen_WatchAdvertisementsOptionsimpl_8435 {
() => {
// Module: crate::features::gen_WatchAdvertisementsOptions
// Provides: {"impl_8435"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl WatchAdvertisementsOptions { # [doc = "Construct a new `WatchAdvertisementsOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WatchAdvertisementsOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "AbortSignal")] # [deprecated = "Use `set_signal()` instead."] pub fn signal (& mut self , val : & AbortSignal) -> & mut Self { self . set_signal (val) ; self } }
};
}
