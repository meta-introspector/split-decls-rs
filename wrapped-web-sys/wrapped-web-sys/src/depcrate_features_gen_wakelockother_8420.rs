// Generated macro for other_8420 (other)
macro_rules! Depcrate_features_gen_WakeLockother_8420 {
() => {
// Module: crate::features::gen_WakeLock
// Provides: {"other_8420"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] # [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WakeLock , typescript_type = "WakeLock")] # [derive (Debug , Clone , PartialEq , Eq)] # [doc = "The `WakeLock` class."] # [doc = ""] # [doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WakeLock)"] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WakeLock`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub type WakeLock ; # [cfg (web_sys_unstable_apis)] # [cfg (feature = "WakeLockType")] # [wasm_bindgen (method , structural , js_class = "WakeLock" , js_name = request)] # [doc = "The `request()` method."] # [doc = ""] # [doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WakeLock/request)"] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WakeLock`, `WakeLockType`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn request (this : & WakeLock , type_ : WakeLockType) -> :: js_sys :: Promise ; }
};
}
