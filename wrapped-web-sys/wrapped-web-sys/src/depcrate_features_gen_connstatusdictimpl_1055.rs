// Generated macro for impl_1055 (impl)
macro_rules! Depcrate_features_gen_ConnStatusDictimpl_1055 {
() => {
// Module: crate::features::gen_ConnStatusDict
// Provides: {"impl_1055"}
// Dependencies: {}
impl ConnStatusDict { # [doc = "Construct a new `ConnStatusDict`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_status()` instead."] pub fn status (& mut self , val : & str) -> & mut Self { self . set_status (val) ; self } }
};
}
