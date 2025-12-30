// Generated macro for impl_5701 (impl)
macro_rules! Depcrate_features_gen_PushSubscriptionOptionsInitimpl_5701 {
() => {
// Module: crate::features::gen_PushSubscriptionOptionsInit
// Provides: {"impl_5701"}
// Dependencies: {}
impl PushSubscriptionOptionsInit { # [doc = "Construct a new `PushSubscriptionOptionsInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_application_server_key()` instead."] pub fn application_server_key (& mut self , val : Option < & :: wasm_bindgen :: JsValue >) -> & mut Self { self . set_application_server_key (val . unwrap_or (& :: wasm_bindgen :: JsValue :: NULL)) ; self } # [deprecated = "Use `set_user_visible_only()` instead."] pub fn user_visible_only (& mut self , val : bool) -> & mut Self { self . set_user_visible_only (val) ; self } }
};
}
