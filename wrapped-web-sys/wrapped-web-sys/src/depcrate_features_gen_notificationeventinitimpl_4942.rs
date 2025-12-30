// Generated macro for impl_4942 (impl)
macro_rules! Depcrate_features_gen_NotificationEventInitimpl_4942 {
() => {
// Module: crate::features::gen_NotificationEventInit
// Provides: {"impl_4942"}
// Dependencies: {}
impl NotificationEventInit { # [cfg (feature = "Notification")] # [doc = "Construct a new `NotificationEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `Notification`, `NotificationEventInit`*"] pub fn new (notification : & Notification) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_notification (notification) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "Notification")] # [deprecated = "Use `set_notification()` instead."] pub fn notification (& mut self , val : & Notification) -> & mut Self { self . set_notification (val) ; self } }
};
}
