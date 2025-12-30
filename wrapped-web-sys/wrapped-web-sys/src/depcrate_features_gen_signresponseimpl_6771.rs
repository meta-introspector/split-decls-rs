// Generated macro for impl_6771 (impl)
macro_rules! Depcrate_features_gen_SignResponseimpl_6771 {
() => {
// Module: crate::features::gen_SignResponse
// Provides: {"impl_6771"}
// Dependencies: {}
impl SignResponse { # [doc = "Construct a new `SignResponse`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `SignResponse`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_client_data()` instead."] pub fn client_data (& mut self , val : & str) -> & mut Self { self . set_client_data (val) ; self } # [deprecated = "Use `set_error_code()` instead."] pub fn error_code (& mut self , val : Option < u16 >) -> & mut Self { self . set_error_code (val) ; self } # [deprecated = "Use `set_error_message()` instead."] pub fn error_message (& mut self , val : Option < & str >) -> & mut Self { self . set_error_message (val) ; self } # [deprecated = "Use `set_key_handle()` instead."] pub fn key_handle (& mut self , val : & str) -> & mut Self { self . set_key_handle (val) ; self } # [deprecated = "Use `set_signature_data()` instead."] pub fn signature_data (& mut self , val : & str) -> & mut Self { self . set_signature_data (val) ; self } }
};
}
