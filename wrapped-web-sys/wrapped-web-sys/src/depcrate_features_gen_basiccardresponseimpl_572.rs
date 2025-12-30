// Generated macro for impl_572 (impl)
macro_rules! Depcrate_features_gen_BasicCardResponseimpl_572 {
() => {
// Module: crate::features::gen_BasicCardResponse
// Provides: {"impl_572"}
// Dependencies: {}
impl BasicCardResponse { # [doc = "Construct a new `BasicCardResponse`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"] pub fn new (card_number : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_card_number (card_number) ; ret } # [cfg (feature = "PaymentAddress")] # [deprecated = "Use `set_billing_address()` instead."] pub fn billing_address (& mut self , val : Option < & PaymentAddress >) -> & mut Self { self . set_billing_address (val) ; self } # [deprecated = "Use `set_card_number()` instead."] pub fn card_number (& mut self , val : & str) -> & mut Self { self . set_card_number (val) ; self } # [deprecated = "Use `set_card_security_code()` instead."] pub fn card_security_code (& mut self , val : & str) -> & mut Self { self . set_card_security_code (val) ; self } # [deprecated = "Use `set_cardholder_name()` instead."] pub fn cardholder_name (& mut self , val : & str) -> & mut Self { self . set_cardholder_name (val) ; self } # [deprecated = "Use `set_expiry_month()` instead."] pub fn expiry_month (& mut self , val : & str) -> & mut Self { self . set_expiry_month (val) ; self } # [deprecated = "Use `set_expiry_year()` instead."] pub fn expiry_year (& mut self , val : & str) -> & mut Self { self . set_expiry_year (val) ; self } }
};
}
