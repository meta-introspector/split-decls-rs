// Generated macro for impl_3231 (impl)
macro_rules! Depcrate_features_gen_GroupedHistoryEventInitimpl_3231 {
() => {
// Module: crate::features::gen_GroupedHistoryEventInit
// Provides: {"impl_3231"}
// Dependencies: {}
impl GroupedHistoryEventInit { # [doc = "Construct a new `GroupedHistoryEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "Element")] # [deprecated = "Use `set_other_browser()` instead."] pub fn other_browser (& mut self , val : Option < & Element >) -> & mut Self { self . set_other_browser (val) ; self } }
};
}
