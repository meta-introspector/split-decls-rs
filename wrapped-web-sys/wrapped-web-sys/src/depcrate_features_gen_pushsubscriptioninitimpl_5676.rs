// Generated macro for impl_5676 (impl)
macro_rules! Depcrate_features_gen_PushSubscriptionInitimpl_5676 {
() => {
// Module: crate::features::gen_PushSubscriptionInit
// Provides: {"impl_5676"}
// Dependencies: {}
impl PushSubscriptionInit { # [doc = "Construct a new `PushSubscriptionInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"] pub fn new (endpoint : & str , scope : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_endpoint (endpoint) ; ret . set_scope (scope) ; ret } # [deprecated = "Use `set_app_server_key()` instead."] pub fn app_server_key (& mut self , val : Option < & :: js_sys :: Object >) -> & mut Self { self . set_app_server_key (val) ; self } # [deprecated = "Use `set_auth_secret()` instead."] pub fn auth_secret (& mut self , val : Option < & :: js_sys :: ArrayBuffer >) -> & mut Self { self . set_auth_secret (val) ; self } # [deprecated = "Use `set_endpoint()` instead."] pub fn endpoint (& mut self , val : & str) -> & mut Self { self . set_endpoint (val) ; self } # [deprecated = "Use `set_p256dh_key()` instead."] pub fn p256dh_key (& mut self , val : Option < & :: js_sys :: ArrayBuffer >) -> & mut Self { self . set_p256dh_key (val) ; self } # [deprecated = "Use `set_scope()` instead."] pub fn scope (& mut self , val : & str) -> & mut Self { self . set_scope (val) ; self } }
};
}
