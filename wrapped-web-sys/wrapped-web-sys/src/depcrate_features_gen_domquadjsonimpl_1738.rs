// Generated macro for impl_1738 (impl)
macro_rules! Depcrate_features_gen_DomQuadJsonimpl_1738 {
() => {
// Module: crate::features::gen_DomQuadJson
// Provides: {"impl_1738"}
// Dependencies: {}
impl DomQuadJson { # [doc = "Construct a new `DomQuadJson`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DomQuadJson`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "DomPoint")] # [deprecated = "Use `set_p1()` instead."] pub fn p1 (& mut self , val : & DomPoint) -> & mut Self { self . set_p1 (val) ; self } # [cfg (feature = "DomPoint")] # [deprecated = "Use `set_p2()` instead."] pub fn p2 (& mut self , val : & DomPoint) -> & mut Self { self . set_p2 (val) ; self } # [cfg (feature = "DomPoint")] # [deprecated = "Use `set_p3()` instead."] pub fn p3 (& mut self , val : & DomPoint) -> & mut Self { self . set_p3 (val) ; self } # [cfg (feature = "DomPoint")] # [deprecated = "Use `set_p4()` instead."] pub fn p4 (& mut self , val : & DomPoint) -> & mut Self { self . set_p4 (val) ; self } }
};
}
