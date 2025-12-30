// Generated macro for impl_4071 (impl)
macro_rules! Depcrate_features_gen_IntersectionObserverInitimpl_4071 {
() => {
// Module: crate::features::gen_IntersectionObserverInit
// Provides: {"impl_4071"}
// Dependencies: {}
impl IntersectionObserverInit { # [doc = "Construct a new `IntersectionObserverInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "Element")] # [deprecated = "Use `set_root()` instead."] pub fn root (& mut self , val : Option < & Element >) -> & mut Self { self . set_root (val) ; self } # [deprecated = "Use `set_root_margin()` instead."] pub fn root_margin (& mut self , val : & str) -> & mut Self { self . set_root_margin (val) ; self } # [deprecated = "Use `set_threshold()` instead."] pub fn threshold (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_threshold (val) ; self } }
};
}
