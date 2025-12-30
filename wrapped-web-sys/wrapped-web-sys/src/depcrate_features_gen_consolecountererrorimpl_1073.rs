// Generated macro for impl_1073 (impl)
macro_rules! Depcrate_features_gen_ConsoleCounterErrorimpl_1073 {
() => {
// Module: crate::features::gen_ConsoleCounterError
// Provides: {"impl_1073"}
// Dependencies: {}
impl ConsoleCounterError { # [doc = "Construct a new `ConsoleCounterError`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_error()` instead."] pub fn error (& mut self , val : & str) -> & mut Self { self . set_error (val) ; self } # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & str) -> & mut Self { self . set_label (val) ; self } }
};
}
