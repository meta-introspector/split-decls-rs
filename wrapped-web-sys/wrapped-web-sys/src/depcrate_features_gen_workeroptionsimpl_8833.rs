// Generated macro for impl_8833 (impl)
macro_rules! Depcrate_features_gen_WorkerOptionsimpl_8833 {
() => {
// Module: crate::features::gen_WorkerOptions
// Provides: {"impl_8833"}
// Dependencies: {}
impl WorkerOptions { # [doc = "Construct a new `WorkerOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WorkerOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "RequestCredentials")] # [deprecated = "Use `set_credentials()` instead."] pub fn credentials (& mut self , val : RequestCredentials) -> & mut Self { self . set_credentials (val) ; self } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [cfg (feature = "WorkerType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : WorkerType) -> & mut Self { self . set_type (val) ; self } }
};
}
