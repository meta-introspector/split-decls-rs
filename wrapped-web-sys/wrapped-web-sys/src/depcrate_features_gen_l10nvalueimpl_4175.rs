// Generated macro for impl_4175 (impl)
macro_rules! Depcrate_features_gen_L10nValueimpl_4175 {
() => {
// Module: crate::features::gen_L10nValue
// Provides: {"impl_4175"}
// Dependencies: {}
impl L10nValue { # [doc = "Construct a new `L10nValue`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `L10nValue`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_attributes()` instead."] pub fn attributes (& mut self , val : Option < & :: wasm_bindgen :: JsValue >) -> & mut Self { self . set_attributes (val . unwrap_or (& :: wasm_bindgen :: JsValue :: NULL)) ; self } # [deprecated = "Use `set_value()` instead."] pub fn value (& mut self , val : Option < & str >) -> & mut Self { self . set_value (val) ; self } }
};
}
