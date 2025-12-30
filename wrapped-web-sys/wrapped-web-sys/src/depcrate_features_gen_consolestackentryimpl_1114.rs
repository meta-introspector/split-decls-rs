// Generated macro for impl_1114 (impl)
macro_rules! Depcrate_features_gen_ConsoleStackEntryimpl_1114 {
() => {
// Module: crate::features::gen_ConsoleStackEntry
// Provides: {"impl_1114"}
// Dependencies: {}
impl ConsoleStackEntry { # [doc = "Construct a new `ConsoleStackEntry`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_async_cause()` instead."] pub fn async_cause (& mut self , val : Option < & str >) -> & mut Self { self . set_async_cause (val) ; self } # [deprecated = "Use `set_column_number()` instead."] pub fn column_number (& mut self , val : u32) -> & mut Self { self . set_column_number (val) ; self } # [deprecated = "Use `set_filename()` instead."] pub fn filename (& mut self , val : & str) -> & mut Self { self . set_filename (val) ; self } # [deprecated = "Use `set_function_name()` instead."] pub fn function_name (& mut self , val : & str) -> & mut Self { self . set_function_name (val) ; self } # [deprecated = "Use `set_line_number()` instead."] pub fn line_number (& mut self , val : u32) -> & mut Self { self . set_line_number (val) ; self } }
};
}
