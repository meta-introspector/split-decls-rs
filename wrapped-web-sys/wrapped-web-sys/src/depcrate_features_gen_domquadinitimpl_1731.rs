// Generated macro for impl_1731 (impl)
macro_rules! Depcrate_features_gen_DomQuadInitimpl_1731 {
() => {
// Module: crate::features::gen_DomQuadInit
// Provides: {"impl_1731"}
// Dependencies: {}
impl DomQuadInit { # [doc = "Construct a new `DomQuadInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DomQuadInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "DomPointInit")] # [deprecated = "Use `set_p1()` instead."] pub fn p1 (& mut self , val : & DomPointInit) -> & mut Self { self . set_p1 (val) ; self } # [cfg (feature = "DomPointInit")] # [deprecated = "Use `set_p2()` instead."] pub fn p2 (& mut self , val : & DomPointInit) -> & mut Self { self . set_p2 (val) ; self } # [cfg (feature = "DomPointInit")] # [deprecated = "Use `set_p3()` instead."] pub fn p3 (& mut self , val : & DomPointInit) -> & mut Self { self . set_p3 (val) ; self } # [cfg (feature = "DomPointInit")] # [deprecated = "Use `set_p4()` instead."] pub fn p4 (& mut self , val : & DomPointInit) -> & mut Self { self . set_p4 (val) ; self } }
};
}
