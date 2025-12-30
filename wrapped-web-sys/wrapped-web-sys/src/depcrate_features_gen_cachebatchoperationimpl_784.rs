// Generated macro for impl_784 (impl)
macro_rules! Depcrate_features_gen_CacheBatchOperationimpl_784 {
() => {
// Module: crate::features::gen_CacheBatchOperation
// Provides: {"impl_784"}
// Dependencies: {}
impl CacheBatchOperation { # [doc = "Construct a new `CacheBatchOperation`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "CacheQueryOptions")] # [deprecated = "Use `set_options()` instead."] pub fn options (& mut self , val : & CacheQueryOptions) -> & mut Self { self . set_options (val) ; self } # [cfg (feature = "Request")] # [deprecated = "Use `set_request()` instead."] pub fn request (& mut self , val : & Request) -> & mut Self { self . set_request (val) ; self } # [cfg (feature = "Response")] # [deprecated = "Use `set_response()` instead."] pub fn response (& mut self , val : & Response) -> & mut Self { self . set_response (val) ; self } # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : & str) -> & mut Self { self . set_type (val) ; self } }
};
}
