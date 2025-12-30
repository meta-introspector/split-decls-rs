// Generated macro for impl_735 (impl)
macro_rules! Depcrate_features_gen_BoxQuadOptionsimpl_735 {
() => {
// Module: crate::features::gen_BoxQuadOptions
// Provides: {"impl_735"}
// Dependencies: {}
impl BoxQuadOptions { # [doc = "Construct a new `BoxQuadOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "CssBoxType")] # [deprecated = "Use `set_box()` instead."] pub fn box_ (& mut self , val : CssBoxType) -> & mut Self { self . set_box (val) ; self } # [deprecated = "Use `set_relative_to()` instead."] pub fn relative_to (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_relative_to (val) ; self } }
};
}
