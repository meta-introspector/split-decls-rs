// Generated macro for impl_6744 (impl)
macro_rules! Depcrate_features_gen_ShadowRootInitimpl_6744 {
() => {
// Module: crate::features::gen_ShadowRootInit
// Provides: {"impl_6744"}
// Dependencies: {}
impl ShadowRootInit { # [cfg (feature = "ShadowRootMode")] # [doc = "Construct a new `ShadowRootInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ShadowRootInit`, `ShadowRootMode`*"] pub fn new (mode : ShadowRootMode) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_mode (mode) ; ret } # [cfg (feature = "ShadowRootMode")] # [deprecated = "Use `set_mode()` instead."] pub fn mode (& mut self , val : ShadowRootMode) -> & mut Self { self . set_mode (val) ; self } }
};
}
