// Generated macro for impl_1128 (impl)
macro_rules! Depcrate_features_gen_ConsoleTimerLogOrEndimpl_1128 {
() => {
// Module: crate::features::gen_ConsoleTimerLogOrEnd
// Provides: {"impl_1128"}
// Dependencies: {}
impl ConsoleTimerLogOrEnd { # [doc = "Construct a new `ConsoleTimerLogOrEnd`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_duration()` instead."] pub fn duration (& mut self , val : f64) -> & mut Self { self . set_duration (val) ; self } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } }
};
}
