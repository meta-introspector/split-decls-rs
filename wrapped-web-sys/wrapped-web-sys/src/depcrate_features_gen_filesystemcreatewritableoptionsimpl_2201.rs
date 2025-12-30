// Generated macro for impl_2201 (impl)
macro_rules! Depcrate_features_gen_FileSystemCreateWritableOptionsimpl_2201 {
() => {
// Module: crate::features::gen_FileSystemCreateWritableOptions
// Provides: {"impl_2201"}
// Dependencies: {}
impl FileSystemCreateWritableOptions { # [doc = "Construct a new `FileSystemCreateWritableOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_keep_existing_data()` instead."] pub fn keep_existing_data (& mut self , val : bool) -> & mut Self { self . set_keep_existing_data (val) ; self } }
};
}
