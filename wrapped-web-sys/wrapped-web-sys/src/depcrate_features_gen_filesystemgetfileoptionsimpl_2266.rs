// Generated macro for impl_2266 (impl)
macro_rules! Depcrate_features_gen_FileSystemGetFileOptionsimpl_2266 {
() => {
// Module: crate::features::gen_FileSystemGetFileOptions
// Provides: {"impl_2266"}
// Dependencies: {}
impl FileSystemGetFileOptions { # [doc = "Construct a new `FileSystemGetFileOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FileSystemGetFileOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_create()` instead."] pub fn create (& mut self , val : bool) -> & mut Self { self . set_create (val) ; self } }
};
}
