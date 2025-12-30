// Generated macro for impl_1107 (impl)
macro_rules! Depcrate_features_gen_ConsoleProfileEventimpl_1107 {
() => {
// Module: crate::features::gen_ConsoleProfileEvent
// Provides: {"impl_1107"}
// Dependencies: {}
impl ConsoleProfileEvent { # [doc = "Construct a new `ConsoleProfileEvent`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_action()` instead."] pub fn action (& mut self , val : & str) -> & mut Self { self . set_action (val) ; self } # [deprecated = "Use `set_arguments()` instead."] pub fn arguments (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_arguments (val) ; self } }
};
}
