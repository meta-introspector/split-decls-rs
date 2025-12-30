// Generated macro for impl_2235 (impl)
macro_rules! Depcrate_features_gen_FileSystemEntryCallbackimpl_2235 {
() => {
// Module: crate::features::gen_FileSystemEntryCallback
// Provides: {"impl_2235"}
// Dependencies: {}
impl FileSystemEntryCallback { # [doc = "Construct a new `FileSystemEntryCallback`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FileSystemEntryCallback`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_handle_event()` instead."] pub fn handle_event (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_handle_event (val) ; self } }
};
}
