// Generated macro for impl_2223 (impl)
macro_rules! Depcrate_features_gen_FileSystemEntriesCallbackimpl_2223 {
() => {
// Module: crate::features::gen_FileSystemEntriesCallback
// Provides: {"impl_2223"}
// Dependencies: {}
impl FileSystemEntriesCallback { # [doc = "Construct a new `FileSystemEntriesCallback`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FileSystemEntriesCallback`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_handle_event()` instead."] pub fn handle_event (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_handle_event (val) ; self } }
};
}
