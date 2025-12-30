// Generated macro for impl_5049 (impl)
macro_rules! Depcrate_features_gen_OpenWindowEventDetailimpl_5049 {
() => {
// Module: crate::features::gen_OpenWindowEventDetail
// Provides: {"impl_5049"}
// Dependencies: {}
impl OpenWindowEventDetail { # [doc = "Construct a new `OpenWindowEventDetail`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_features()` instead."] pub fn features (& mut self , val : & str) -> & mut Self { self . set_features (val) ; self } # [cfg (feature = "Node")] # [deprecated = "Use `set_frame_element()` instead."] pub fn frame_element (& mut self , val : Option < & Node >) -> & mut Self { self . set_frame_element (val) ; self } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_url()` instead."] pub fn url (& mut self , val : & str) -> & mut Self { self . set_url (val) ; self } }
};
}
