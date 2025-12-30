// Generated macro for impl_5523 (impl)
macro_rules! Depcrate_features_gen_ProfileTimelineStackFrameimpl_5523 {
() => {
// Module: crate::features::gen_ProfileTimelineStackFrame
// Provides: {"impl_5523"}
// Dependencies: {}
impl ProfileTimelineStackFrame { # [doc = "Construct a new `ProfileTimelineStackFrame`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_async_cause()` instead."] pub fn async_cause (& mut self , val : & str) -> & mut Self { self . set_async_cause (val) ; self } # [deprecated = "Use `set_async_parent()` instead."] pub fn async_parent (& mut self , val : Option < & :: js_sys :: Object >) -> & mut Self { self . set_async_parent (val) ; self } # [deprecated = "Use `set_column()` instead."] pub fn column (& mut self , val : i32) -> & mut Self { self . set_column (val) ; self } # [deprecated = "Use `set_function_display_name()` instead."] pub fn function_display_name (& mut self , val : & str) -> & mut Self { self . set_function_display_name (val) ; self } # [deprecated = "Use `set_line()` instead."] pub fn line (& mut self , val : i32) -> & mut Self { self . set_line (val) ; self } # [deprecated = "Use `set_parent()` instead."] pub fn parent (& mut self , val : Option < & :: js_sys :: Object >) -> & mut Self { self . set_parent (val) ; self } # [deprecated = "Use `set_source()` instead."] pub fn source (& mut self , val : & str) -> & mut Self { self . set_source (val) ; self } }
};
}
