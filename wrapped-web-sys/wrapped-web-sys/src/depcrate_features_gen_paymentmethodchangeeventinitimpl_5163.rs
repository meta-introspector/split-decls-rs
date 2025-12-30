// Generated macro for impl_5163 (impl)
macro_rules! Depcrate_features_gen_PaymentMethodChangeEventInitimpl_5163 {
() => {
// Module: crate::features::gen_PaymentMethodChangeEventInit
// Provides: {"impl_5163"}
// Dependencies: {}
impl PaymentMethodChangeEventInit { # [doc = "Construct a new `PaymentMethodChangeEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"] pub fn new (method_name : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_method_name (method_name) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_method_details()` instead."] pub fn method_details (& mut self , val : Option < & :: js_sys :: Object >) -> & mut Self { self . set_method_details (val) ; self } # [deprecated = "Use `set_method_name()` instead."] pub fn method_name (& mut self , val : & str) -> & mut Self { self . set_method_name (val) ; self } }
};
}
