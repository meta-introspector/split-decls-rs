// Generated macro for impl_3821 (impl)
macro_rules! Depcrate_features_gen_IdbFileMetadataParametersimpl_3821 {
() => {
// Module: crate::features::gen_IdbFileMetadataParameters
// Provides: {"impl_3821"}
// Dependencies: {}
impl IdbFileMetadataParameters { # [doc = "Construct a new `IdbFileMetadataParameters`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"] # [deprecated] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_last_modified()` instead."] pub fn last_modified (& mut self , val : bool) -> & mut Self { self . set_last_modified (val) ; self } # [deprecated = "Use `set_size()` instead."] pub fn size (& mut self , val : bool) -> & mut Self { self . set_size (val) ; self } }
};
}
