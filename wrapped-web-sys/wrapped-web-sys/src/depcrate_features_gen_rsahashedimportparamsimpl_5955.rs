// Generated macro for impl_5955 (impl)
macro_rules! Depcrate_features_gen_RsaHashedImportParamsimpl_5955 {
() => {
// Module: crate::features::gen_RsaHashedImportParams
// Provides: {"impl_5955"}
// Dependencies: {}
impl RsaHashedImportParams { # [doc = "Construct a new `RsaHashedImportParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"] pub fn new (hash : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_hash (hash) ; ret } # [deprecated = "Use `set_hash()` instead."] pub fn hash (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_hash (val) ; self } }
};
}
