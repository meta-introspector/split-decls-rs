// Generated macro for impl_2385 (impl)
macro_rules! Depcrate_features_gen_FontFaceSetIteratorResultimpl_2385 {
() => {
// Module: crate::features::gen_FontFaceSetIteratorResult
// Provides: {"impl_2385"}
// Dependencies: {}
impl FontFaceSetIteratorResult { # [doc = "Construct a new `FontFaceSetIteratorResult`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"] pub fn new (done : bool , value : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_done (done) ; ret . set_value (value) ; ret } # [deprecated = "Use `set_done()` instead."] pub fn done (& mut self , val : bool) -> & mut Self { self . set_done (val) ; self } # [deprecated = "Use `set_value()` instead."] pub fn value (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_value (val) ; self } }
};
}
