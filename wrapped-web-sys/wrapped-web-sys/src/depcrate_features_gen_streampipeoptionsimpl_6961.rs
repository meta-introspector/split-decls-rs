// Generated macro for impl_6961 (impl)
macro_rules! Depcrate_features_gen_StreamPipeOptionsimpl_6961 {
() => {
// Module: crate::features::gen_StreamPipeOptions
// Provides: {"impl_6961"}
// Dependencies: {}
impl StreamPipeOptions { # [doc = "Construct a new `StreamPipeOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_prevent_abort()` instead."] pub fn prevent_abort (& mut self , val : bool) -> & mut Self { self . set_prevent_abort (val) ; self } # [deprecated = "Use `set_prevent_cancel()` instead."] pub fn prevent_cancel (& mut self , val : bool) -> & mut Self { self . set_prevent_cancel (val) ; self } # [deprecated = "Use `set_prevent_close()` instead."] pub fn prevent_close (& mut self , val : bool) -> & mut Self { self . set_prevent_close (val) ; self } # [cfg (feature = "AbortSignal")] # [deprecated = "Use `set_signal()` instead."] pub fn signal (& mut self , val : & AbortSignal) -> & mut Self { self . set_signal (val) ; self } }
};
}
