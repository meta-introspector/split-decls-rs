// Generated macro for impl_3872 (impl)
macro_rules! Depcrate_features_gen_IdbOpenDbOptionsimpl_3872 {
() => {
// Module: crate::features::gen_IdbOpenDbOptions
// Provides: {"impl_3872"}
// Dependencies: {}
impl IdbOpenDbOptions { # [doc = "Construct a new `IdbOpenDbOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"] # [deprecated] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "StorageType")] # [deprecated = "Use `set_storage()` instead."] pub fn storage (& mut self , val : StorageType) -> & mut Self { self . set_storage (val) ; self } # [deprecated = "Use `set_version()` instead."] pub fn version (& mut self , val : f64) -> & mut Self { self . set_version (val) ; self } }
};
}
