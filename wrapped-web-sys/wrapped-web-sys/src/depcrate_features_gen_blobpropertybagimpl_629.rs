// Generated macro for impl_629 (impl)
macro_rules! Depcrate_features_gen_BlobPropertyBagimpl_629 {
() => {
// Module: crate::features::gen_BlobPropertyBag
// Provides: {"impl_629"}
// Dependencies: {}
impl BlobPropertyBag { # [doc = "Construct a new `BlobPropertyBag`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "EndingTypes")] # [deprecated = "Use `set_endings()` instead."] pub fn endings (& mut self , val : EndingTypes) -> & mut Self { self . set_endings (val) ; self } # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : & str) -> & mut Self { self . set_type (val) ; self } }
};
}
