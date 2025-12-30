// Generated macro for impl_9071 (impl)
macro_rules! Depcrate_features_gen_XrSessionEventInitimpl_9071 {
() => {
// Module: crate::features::gen_XrSessionEventInit
// Provides: {"impl_9071"}
// Dependencies: {}
# [cfg (web_sys_unstable_apis)] impl XrSessionEventInit { # [cfg (feature = "XrSession")] # [doc = "Construct a new `XrSessionEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `XrSession`, `XrSessionEventInit`*"] # [doc = ""] # [doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"] # [doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"] pub fn new (session : & XrSession) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_session (session) ; ret } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [cfg (web_sys_unstable_apis)] # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (web_sys_unstable_apis)] # [cfg (feature = "XrSession")] # [deprecated = "Use `set_session()` instead."] pub fn session (& mut self , val : & XrSession) -> & mut Self { self . set_session (val) ; self } }
};
}
