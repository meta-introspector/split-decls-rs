// Generated macro for impl_2152 (impl)
macro_rules! Depcrate_features_gen_FileCallbackimpl_2152 {
() => {
// Module: crate::features::gen_FileCallback
// Provides: {"impl_2152"}
// Dependencies: {}
impl FileCallback { # [doc = "Construct a new `FileCallback`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FileCallback`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_handle_event()` instead."] pub fn handle_event (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_handle_event (val) ; self } }
};
}
