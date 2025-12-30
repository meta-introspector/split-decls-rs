// Generated macro for impl_6144 (impl)
macro_rules! Depcrate_features_gen_RtcIceServerimpl_6144 {
() => {
// Module: crate::features::gen_RtcIceServer
// Provides: {"impl_6144"}
// Dependencies: {}
impl RtcIceServer { # [doc = "Construct a new `RtcIceServer`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_credential()` instead."] pub fn credential (& mut self , val : & str) -> & mut Self { self . set_credential (val) ; self } # [cfg (feature = "RtcIceCredentialType")] # [deprecated = "Use `set_credential_type()` instead."] pub fn credential_type (& mut self , val : RtcIceCredentialType) -> & mut Self { self . set_credential_type (val) ; self } # [deprecated = "Use `set_url()` instead."] pub fn url (& mut self , val : & str) -> & mut Self { self . set_url (val) ; self } # [deprecated = "Use `set_urls()` instead."] pub fn urls (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_urls (val) ; self } # [deprecated = "Use `set_username()` instead."] pub fn username (& mut self , val : & str) -> & mut Self { self . set_username (val) ; self } }
};
}
