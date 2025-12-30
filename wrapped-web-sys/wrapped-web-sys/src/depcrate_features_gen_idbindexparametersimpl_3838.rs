// Generated macro for impl_3838 (impl)
macro_rules! Depcrate_features_gen_IdbIndexParametersimpl_3838 {
() => {
// Module: crate::features::gen_IdbIndexParameters
// Provides: {"impl_3838"}
// Dependencies: {}
impl IdbIndexParameters { # [doc = "Construct a new `IdbIndexParameters`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_locale()` instead."] pub fn locale (& mut self , val : Option < & str >) -> & mut Self { self . set_locale (val) ; self } # [deprecated = "Use `set_multi_entry()` instead."] pub fn multi_entry (& mut self , val : bool) -> & mut Self { self . set_multi_entry (val) ; self } # [deprecated = "Use `set_unique()` instead."] pub fn unique (& mut self , val : bool) -> & mut Self { self . set_unique (val) ; self } }
};
}
