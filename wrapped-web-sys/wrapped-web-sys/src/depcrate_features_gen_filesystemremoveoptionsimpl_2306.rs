// Generated macro for impl_2306 (impl)
macro_rules! Depcrate_features_gen_FileSystemRemoveOptionsimpl_2306 {
() => {
// Module: crate::features::gen_FileSystemRemoveOptions
// Provides: {"impl_2306"}
// Dependencies: {}
impl FileSystemRemoveOptions { # [doc = "Construct a new `FileSystemRemoveOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FileSystemRemoveOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_recursive()` instead."] pub fn recursive (& mut self , val : bool) -> & mut Self { self . set_recursive (val) ; self } }
};
}
