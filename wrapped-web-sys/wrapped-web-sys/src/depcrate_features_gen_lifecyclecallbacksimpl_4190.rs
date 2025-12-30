// Generated macro for impl_4190 (impl)
macro_rules! Depcrate_features_gen_LifecycleCallbacksimpl_4190 {
() => {
// Module: crate::features::gen_LifecycleCallbacks
// Provides: {"impl_4190"}
// Dependencies: {}
impl LifecycleCallbacks { # [doc = "Construct a new `LifecycleCallbacks`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_adopted_callback()` instead."] pub fn adopted_callback (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_adopted_callback (val) ; self } # [deprecated = "Use `set_attribute_changed_callback()` instead."] pub fn attribute_changed_callback (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_attribute_changed_callback (val) ; self } # [deprecated = "Use `set_connected_callback()` instead."] pub fn connected_callback (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_connected_callback (val) ; self } # [deprecated = "Use `set_disconnected_callback()` instead."] pub fn disconnected_callback (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_disconnected_callback (val) ; self } }
};
}
