// Generated macro for impl_5225 (impl)
macro_rules! Depcrate_features_gen_PerformanceEntryFilterOptionsimpl_5225 {
() => {
// Module: crate::features::gen_PerformanceEntryFilterOptions
// Provides: {"impl_5225"}
// Dependencies: {}
impl PerformanceEntryFilterOptions { # [doc = "Construct a new `PerformanceEntryFilterOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_entry_type()` instead."] pub fn entry_type (& mut self , val : & str) -> & mut Self { self . set_entry_type (val) ; self } # [deprecated = "Use `set_initiator_type()` instead."] pub fn initiator_type (& mut self , val : & str) -> & mut Self { self . set_initiator_type (val) ; self } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } }
};
}
