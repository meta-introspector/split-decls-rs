// Generated macro for impl_4927 (impl)
macro_rules! Depcrate_features_gen_NotificationActionimpl_4927 {
() => {
// Module: crate::features::gen_NotificationAction
// Provides: {"impl_4927"}
// Dependencies: {}
impl NotificationAction { # [doc = "Construct a new `NotificationAction`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `NotificationAction`*"] pub fn new (action : & str , title : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_action (action) ; ret . set_title (title) ; ret } # [deprecated = "Use `set_action()` instead."] pub fn action (& mut self , val : & str) -> & mut Self { self . set_action (val) ; self } # [deprecated = "Use `set_icon()` instead."] pub fn icon (& mut self , val : & str) -> & mut Self { self . set_icon (val) ; self } # [deprecated = "Use `set_title()` instead."] pub fn title (& mut self , val : & str) -> & mut Self { self . set_title (val) ; self } }
};
}
