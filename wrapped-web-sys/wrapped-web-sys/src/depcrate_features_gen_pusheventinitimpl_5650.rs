// Generated macro for impl_5650 (impl)
macro_rules! Depcrate_features_gen_PushEventInitimpl_5650 {
() => {
// Module: crate::features::gen_PushEventInit
// Provides: {"impl_5650"}
// Dependencies: {}
impl PushEventInit { # [doc = "Construct a new `PushEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PushEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_data()` instead."] pub fn data (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_data (val) ; self } }
};
}
