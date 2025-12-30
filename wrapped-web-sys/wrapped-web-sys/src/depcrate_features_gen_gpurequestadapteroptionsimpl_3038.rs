// Generated macro for impl_3038 (impl)
macro_rules! Depcrate_features_gen_GpuRequestAdapterOptionsimpl_3038 {
() => {
// Module: crate::features::gen_GpuRequestAdapterOptions
// Provides: {"impl_3038"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl GpuRequestAdapterOptions { # [doc = "Construct a new `GpuRequestAdapterOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GpuRequestAdapterOptions`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_feature_level()` instead."] pub fn feature_level (& mut self , val : & str) -> & mut Self { self . set_feature_level (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_force_fallback_adapter()` instead."] pub fn force_fallback_adapter (& mut self , val : bool) -> & mut Self { self . set_force_fallback_adapter (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "GpuPowerPreference")] # [deprecated = "Use `set_power_preference()` instead."] pub fn power_preference (& mut self , val : GpuPowerPreference) -> & mut Self { self . set_power_preference (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_xr_compatible()` instead."] pub fn xr_compatible (& mut self , val : bool) -> & mut Self { self . set_xr_compatible (val) ; self } }
};
}
