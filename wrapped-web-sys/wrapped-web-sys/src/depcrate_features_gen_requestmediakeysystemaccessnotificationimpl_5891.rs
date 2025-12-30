// Generated macro for impl_5891 (impl)
macro_rules! Depcrate_features_gen_RequestMediaKeySystemAccessNotificationimpl_5891 {
() => {
// Module: crate::features::gen_RequestMediaKeySystemAccessNotification
// Provides: {"impl_5891"}
// Dependencies: {}
impl RequestMediaKeySystemAccessNotification { # [cfg (feature = "MediaKeySystemStatus")] # [doc = "Construct a new `RequestMediaKeySystemAccessNotification`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaKeySystemStatus`, `RequestMediaKeySystemAccessNotification`*"] pub fn new (key_system : & str , status : MediaKeySystemStatus) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_key_system (key_system) ; ret . set_status (status) ; ret } # [deprecated = "Use `set_key_system()` instead."] pub fn key_system (& mut self , val : & str) -> & mut Self { self . set_key_system (val) ; self } # [cfg (feature = "MediaKeySystemStatus")] # [deprecated = "Use `set_status()` instead."] pub fn status (& mut self , val : MediaKeySystemStatus) -> & mut Self { self . set_status (val) ; self } }
};
}
