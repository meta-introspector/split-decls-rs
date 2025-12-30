// Generated macro for impl_1630 (impl)
macro_rules! Depcrate_features_gen_DnsLookupDictimpl_1630 {
() => {
// Module: crate::features::gen_DnsLookupDict
// Provides: {"impl_1630"}
// Dependencies: {}
impl DnsLookupDict { # [doc = "Construct a new `DnsLookupDict`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_address()` instead."] pub fn address (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_address (val) ; self } # [deprecated = "Use `set_answer()` instead."] pub fn answer (& mut self , val : bool) -> & mut Self { self . set_answer (val) ; self } # [deprecated = "Use `set_error()` instead."] pub fn error (& mut self , val : & str) -> & mut Self { self . set_error (val) ; self } }
};
}
