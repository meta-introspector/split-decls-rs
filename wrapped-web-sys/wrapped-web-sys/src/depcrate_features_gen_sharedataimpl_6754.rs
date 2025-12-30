// Generated macro for impl_6754 (impl)
macro_rules! Depcrate_features_gen_ShareDataimpl_6754 {
() => {
// Module: crate::features::gen_ShareData
// Provides: {"impl_6754"}
// Dependencies: {}
impl ShareData { # [doc = "Construct a new `ShareData`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ShareData`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_files()` instead."] pub fn files (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_files (val) ; self } # [deprecated = "Use `set_text()` instead."] pub fn text (& mut self , val : & str) -> & mut Self { self . set_text (val) ; self } # [deprecated = "Use `set_title()` instead."] pub fn title (& mut self , val : & str) -> & mut Self { self . set_title (val) ; self } # [deprecated = "Use `set_url()` instead."] pub fn url (& mut self , val : & str) -> & mut Self { self . set_url (val) ; self } }
};
}
