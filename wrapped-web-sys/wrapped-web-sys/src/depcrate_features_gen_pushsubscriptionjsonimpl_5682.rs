// Generated macro for impl_5682 (impl)
macro_rules! Depcrate_features_gen_PushSubscriptionJsonimpl_5682 {
() => {
// Module: crate::features::gen_PushSubscriptionJson
// Provides: {"impl_5682"}
// Dependencies: {}
impl PushSubscriptionJson { # [doc = "Construct a new `PushSubscriptionJson`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_endpoint()` instead."] pub fn endpoint (& mut self , val : & str) -> & mut Self { self . set_endpoint (val) ; self } # [cfg (feature = "PushSubscriptionKeys")] # [deprecated = "Use `set_keys()` instead."] pub fn keys (& mut self , val : & PushSubscriptionKeys) -> & mut Self { self . set_keys (val) ; self } }
};
}
