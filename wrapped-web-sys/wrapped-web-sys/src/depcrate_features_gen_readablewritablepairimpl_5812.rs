// Generated macro for impl_5812 (impl)
macro_rules! Depcrate_features_gen_ReadableWritablePairimpl_5812 {
() => {
// Module: crate::features::gen_ReadableWritablePair
// Provides: {"impl_5812"}
// Dependencies: {}
impl ReadableWritablePair { # [cfg (all (feature = "ReadableStream" , feature = "WritableStream" ,))] # [doc = "Construct a new `ReadableWritablePair`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`, `WritableStream`*"] pub fn new (readable : & ReadableStream , writable : & WritableStream) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_readable (readable) ; ret . set_writable (writable) ; ret } # [cfg (feature = "ReadableStream")] # [deprecated = "Use `set_readable()` instead."] pub fn readable (& mut self , val : & ReadableStream) -> & mut Self { self . set_readable (val) ; self } # [cfg (feature = "WritableStream")] # [deprecated = "Use `set_writable()` instead."] pub fn writable (& mut self , val : & WritableStream) -> & mut Self { self . set_writable (val) ; self } }
};
}
