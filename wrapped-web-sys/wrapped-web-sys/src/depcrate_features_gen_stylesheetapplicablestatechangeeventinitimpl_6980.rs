// Generated macro for impl_6980 (impl)
macro_rules! Depcrate_features_gen_StyleSheetApplicableStateChangeEventInitimpl_6980 {
() => {
// Module: crate::features::gen_StyleSheetApplicableStateChangeEventInit
// Provides: {"impl_6980"}
// Dependencies: {}
impl StyleSheetApplicableStateChangeEventInit { # [doc = "Construct a new `StyleSheetApplicableStateChangeEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `StyleSheetApplicableStateChangeEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_applicable()` instead."] pub fn applicable (& mut self , val : bool) -> & mut Self { self . set_applicable (val) ; self } # [cfg (feature = "CssStyleSheet")] # [deprecated = "Use `set_stylesheet()` instead."] pub fn stylesheet (& mut self , val : Option < & CssStyleSheet >) -> & mut Self { self . set_stylesheet (val) ; self } }
};
}
