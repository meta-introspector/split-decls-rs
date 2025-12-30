// Generated macro for impl_5263 (impl)
macro_rules! Depcrate_features_gen_PerformanceObserverInitimpl_5263 {
() => {
// Module: crate::features::gen_PerformanceObserverInit
// Provides: {"impl_5263"}
// Dependencies: {}
impl PerformanceObserverInit { # [doc = "Construct a new `PerformanceObserverInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"] pub fn new (entry_types : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_entry_types (entry_types) ; ret } # [deprecated = "Use `set_buffered()` instead."] pub fn buffered (& mut self , val : bool) -> & mut Self { self . set_buffered (val) ; self } # [deprecated = "Use `set_entry_types()` instead."] pub fn entry_types (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_entry_types (val) ; self } }
};
}
