// Generated macro for impl_5783 (impl)
macro_rules! Depcrate_features_gen_ReadableStreamGetReaderOptionsimpl_5783 {
() => {
// Module: crate::features::gen_ReadableStreamGetReaderOptions
// Provides: {"impl_5783"}
// Dependencies: {}
impl ReadableStreamGetReaderOptions { # [doc = "Construct a new `ReadableStreamGetReaderOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "ReadableStreamReaderMode")] # [deprecated = "Use `set_mode()` instead."] pub fn mode (& mut self , val : ReadableStreamReaderMode) -> & mut Self { self . set_mode (val) ; self } }
};
}
