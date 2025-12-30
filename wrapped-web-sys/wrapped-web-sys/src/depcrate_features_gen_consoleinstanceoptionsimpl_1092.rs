// Generated macro for impl_1092 (impl)
macro_rules! Depcrate_features_gen_ConsoleInstanceOptionsimpl_1092 {
() => {
// Module: crate::features::gen_ConsoleInstanceOptions
// Provides: {"impl_1092"}
// Dependencies: {}
impl ConsoleInstanceOptions { # [doc = "Construct a new `ConsoleInstanceOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_console_id()` instead."] pub fn console_id (& mut self , val : & str) -> & mut Self { self . set_console_id (val) ; self } # [deprecated = "Use `set_dump()` instead."] pub fn dump (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_dump (val) ; self } # [deprecated = "Use `set_inner_id()` instead."] pub fn inner_id (& mut self , val : & str) -> & mut Self { self . set_inner_id (val) ; self } # [cfg (feature = "ConsoleLogLevel")] # [deprecated = "Use `set_max_log_level()` instead."] pub fn max_log_level (& mut self , val : ConsoleLogLevel) -> & mut Self { self . set_max_log_level (val) ; self } # [deprecated = "Use `set_max_log_level_pref()` instead."] pub fn max_log_level_pref (& mut self , val : & str) -> & mut Self { self . set_max_log_level_pref (val) ; self } # [deprecated = "Use `set_prefix()` instead."] pub fn prefix (& mut self , val : & str) -> & mut Self { self . set_prefix (val) ; self } }
};
}
