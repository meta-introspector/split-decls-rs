// Generated macro for impl_5961 (impl)
macro_rules! Depcrate_features_gen_RsaOaepParamsimpl_5961 {
() => {
// Module: crate::features::gen_RsaOaepParams
// Provides: {"impl_5961"}
// Dependencies: {}
impl RsaOaepParams { # [doc = "Construct a new `RsaOaepParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"] pub fn new (name : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_label()` instead."] pub fn label (& mut self , val : & :: js_sys :: Object) -> & mut Self { self . set_label (val) ; self } }
};
}
