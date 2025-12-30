// Generated macro for impl_527 (impl)
macro_rules! Depcrate_features_gen_AutocompleteInfoimpl_527 {
() => {
// Module: crate::features::gen_AutocompleteInfo
// Provides: {"impl_527"}
// Dependencies: {}
impl AutocompleteInfo { # [doc = "Construct a new `AutocompleteInfo`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_address_type()` instead."] pub fn address_type (& mut self , val : & str) -> & mut Self { self . set_address_type (val) ; self } # [deprecated = "Use `set_contact_type()` instead."] pub fn contact_type (& mut self , val : & str) -> & mut Self { self . set_contact_type (val) ; self } # [deprecated = "Use `set_field_name()` instead."] pub fn field_name (& mut self , val : & str) -> & mut Self { self . set_field_name (val) ; self } # [deprecated = "Use `set_section()` instead."] pub fn section (& mut self , val : & str) -> & mut Self { self . set_section (val) ; self } }
};
}
