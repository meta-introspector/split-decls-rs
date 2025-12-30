// Generated macro for impl_1121 (impl)
macro_rules! Depcrate_features_gen_ConsoleTimerErrorimpl_1121 {
() => {
// Module: crate::features::gen_ConsoleTimerError
// Provides: {"impl_1121"}
// Dependencies: {}
impl ConsoleTimerError { # [doc = "Construct a new `ConsoleTimerError`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_error()` instead."] pub fn error (& mut self , val : & str) -> & mut Self { self . set_error (val) ; self } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } }
};
}
