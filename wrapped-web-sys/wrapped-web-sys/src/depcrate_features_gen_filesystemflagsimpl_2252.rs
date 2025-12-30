// Generated macro for impl_2252 (impl)
macro_rules! Depcrate_features_gen_FileSystemFlagsimpl_2252 {
() => {
// Module: crate::features::gen_FileSystemFlags
// Provides: {"impl_2252"}
// Dependencies: {}
impl FileSystemFlags { # [doc = "Construct a new `FileSystemFlags`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_create()` instead."] pub fn create (& mut self , val : bool) -> & mut Self { self . set_create (val) ; self } # [deprecated = "Use `set_exclusive()` instead."] pub fn exclusive (& mut self , val : bool) -> & mut Self { self . set_exclusive (val) ; self } }
};
}
