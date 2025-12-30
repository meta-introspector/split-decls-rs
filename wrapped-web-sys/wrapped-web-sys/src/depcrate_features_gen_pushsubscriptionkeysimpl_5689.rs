// Generated macro for impl_5689 (impl)
macro_rules! Depcrate_features_gen_PushSubscriptionKeysimpl_5689 {
() => {
// Module: crate::features::gen_PushSubscriptionKeys
// Provides: {"impl_5689"}
// Dependencies: {}
impl PushSubscriptionKeys { # [doc = "Construct a new `PushSubscriptionKeys`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_auth()` instead."] pub fn auth (& mut self , val : & str) -> & mut Self { self . set_auth (val) ; self } # [deprecated = "Use `set_p256dh()` instead."] pub fn p256dh (& mut self , val : & str) -> & mut Self { self . set_p256dh (val) ; self } }
};
}
