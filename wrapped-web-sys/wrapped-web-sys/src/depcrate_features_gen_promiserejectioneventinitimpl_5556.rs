// Generated macro for impl_5556 (impl)
macro_rules! Depcrate_features_gen_PromiseRejectionEventInitimpl_5556 {
() => {
// Module: crate::features::gen_PromiseRejectionEventInit
// Provides: {"impl_5556"}
// Dependencies: {}
impl PromiseRejectionEventInit { # [doc = "Construct a new `PromiseRejectionEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"] pub fn new (promise : & :: js_sys :: Promise) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_promise (promise) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_promise()` instead."] pub fn promise (& mut self , val : & :: js_sys :: Promise) -> & mut Self { self . set_promise (val) ; self } # [deprecated = "Use `set_reason()` instead."] pub fn reason (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_reason (val) ; self } }
};
}
