// Generated macro for impl_5833 (impl)
macro_rules! Depcrate_features_gen_RegisterResponseimpl_5833 {
() => {
// Module: crate::features::gen_RegisterResponse
// Provides: {"impl_5833"}
// Dependencies: {}
impl RegisterResponse { # [doc = "Construct a new `RegisterResponse`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_client_data()` instead."] pub fn client_data (& mut self , val : & str) -> & mut Self { self . set_client_data (val) ; self } # [deprecated = "Use `set_error_code()` instead."] pub fn error_code (& mut self , val : Option < u16 >) -> & mut Self { self . set_error_code (val) ; self } # [deprecated = "Use `set_error_message()` instead."] pub fn error_message (& mut self , val : Option < & str >) -> & mut Self { self . set_error_message (val) ; self } # [deprecated = "Use `set_registration_data()` instead."] pub fn registration_data (& mut self , val : & str) -> & mut Self { self . set_registration_data (val) ; self } # [deprecated = "Use `set_version()` instead."] pub fn version (& mut self , val : & str) -> & mut Self { self . set_version (val) ; self } }
};
}
