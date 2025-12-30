// Generated macro for impl_2101 (impl)
macro_rules! Depcrate_features_gen_FakePluginMimeEntryimpl_2101 {
() => {
// Module: crate::features::gen_FakePluginMimeEntry
// Provides: {"impl_2101"}
// Dependencies: {}
impl FakePluginMimeEntry { # [doc = "Construct a new `FakePluginMimeEntry`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"] pub fn new (type_ : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_type (type_) ; ret } # [deprecated = "Use `set_description()` instead."] pub fn description (& mut self , val : & str) -> & mut Self { self . set_description (val) ; self } # [deprecated = "Use `set_extension()` instead."] pub fn extension (& mut self , val : & str) -> & mut Self { self . set_extension (val) ; self } # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : & str) -> & mut Self { self . set_type (val) ; self } }
};
}
