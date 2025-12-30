// Generated macro for impl_8002 (impl)
macro_rules! Depcrate_features_gen_UnderlyingSourceimpl_8002 {
() => {
// Module: crate::features::gen_UnderlyingSource
// Provides: {"impl_8002"}
// Dependencies: {}
impl UnderlyingSource { # [doc = "Construct a new `UnderlyingSource`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_auto_allocate_chunk_size()` instead."] pub fn auto_allocate_chunk_size (& mut self , val : f64) -> & mut Self { self . set_auto_allocate_chunk_size (val) ; self } # [deprecated = "Use `set_cancel()` instead."] pub fn cancel (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_cancel (val) ; self } # [deprecated = "Use `set_pull()` instead."] pub fn pull (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_pull (val) ; self } # [deprecated = "Use `set_start()` instead."] pub fn start (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_start (val) ; self } # [cfg (feature = "ReadableStreamType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : ReadableStreamType) -> & mut Self { self . set_type (val) ; self } }
};
}
