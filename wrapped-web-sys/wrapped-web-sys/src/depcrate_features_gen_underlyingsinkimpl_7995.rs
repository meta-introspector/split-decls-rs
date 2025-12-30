// Generated macro for impl_7995 (impl)
macro_rules! Depcrate_features_gen_UnderlyingSinkimpl_7995 {
() => {
// Module: crate::features::gen_UnderlyingSink
// Provides: {"impl_7995"}
// Dependencies: {}
impl UnderlyingSink { # [doc = "Construct a new `UnderlyingSink`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_abort()` instead."] pub fn abort (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_abort (val) ; self } # [deprecated = "Use `set_close()` instead."] pub fn close (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_close (val) ; self } # [deprecated = "Use `set_start()` instead."] pub fn start (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_start (val) ; self } # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_type (val) ; self } # [deprecated = "Use `set_write()` instead."] pub fn write (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_write (val) ; self } }
};
}
