// Generated macro for impl_5923 (impl)
macro_rules! Depcrate_features_gen_ResizeObserverOptionsimpl_5923 {
() => {
// Module: crate::features::gen_ResizeObserverOptions
// Provides: {"impl_5923"}
// Dependencies: {}
impl ResizeObserverOptions { # [doc = "Construct a new `ResizeObserverOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ResizeObserverOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "ResizeObserverBoxOptions")] # [deprecated = "Use `set_box()` instead."] pub fn box_ (& mut self , val : ResizeObserverBoxOptions) -> & mut Self { self . set_box (val) ; self } }
};
}
