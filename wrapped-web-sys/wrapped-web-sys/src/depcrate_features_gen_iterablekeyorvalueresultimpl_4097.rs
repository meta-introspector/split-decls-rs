// Generated macro for impl_4097 (impl)
macro_rules! Depcrate_features_gen_IterableKeyOrValueResultimpl_4097 {
() => {
// Module: crate::features::gen_IterableKeyOrValueResult
// Provides: {"impl_4097"}
// Dependencies: {}
impl IterableKeyOrValueResult { # [doc = "Construct a new `IterableKeyOrValueResult`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_done()` instead."] pub fn done (& mut self , val : bool) -> & mut Self { self . set_done (val) ; self } # [deprecated = "Use `set_value()` instead."] pub fn value (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_value (val) ; self } }
};
}
