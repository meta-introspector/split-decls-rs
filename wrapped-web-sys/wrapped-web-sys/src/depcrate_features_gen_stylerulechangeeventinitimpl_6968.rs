// Generated macro for impl_6968 (impl)
macro_rules! Depcrate_features_gen_StyleRuleChangeEventInitimpl_6968 {
() => {
// Module: crate::features::gen_StyleRuleChangeEventInit
// Provides: {"impl_6968"}
// Dependencies: {}
impl StyleRuleChangeEventInit { # [doc = "Construct a new `StyleRuleChangeEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "CssRule")] # [deprecated = "Use `set_rule()` instead."] pub fn rule (& mut self , val : Option < & CssRule >) -> & mut Self { self . set_rule (val) ; self } # [cfg (feature = "CssStyleSheet")] # [deprecated = "Use `set_stylesheet()` instead."] pub fn stylesheet (& mut self , val : Option < & CssStyleSheet >) -> & mut Self { self . set_stylesheet (val) ; self } }
};
}
