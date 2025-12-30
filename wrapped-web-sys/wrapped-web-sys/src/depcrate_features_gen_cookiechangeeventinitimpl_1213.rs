// Generated macro for impl_1213 (impl)
macro_rules! Depcrate_features_gen_CookieChangeEventInitimpl_1213 {
() => {
// Module: crate::features::gen_CookieChangeEventInit
// Provides: {"impl_1213"}
// Dependencies: {}
impl CookieChangeEventInit { # [doc = "Construct a new `CookieChangeEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CookieChangeEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_changed()` instead."] pub fn changed (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_changed (val) ; self } # [deprecated = "Use `set_deleted()` instead."] pub fn deleted (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_deleted (val) ; self } }
};
}
