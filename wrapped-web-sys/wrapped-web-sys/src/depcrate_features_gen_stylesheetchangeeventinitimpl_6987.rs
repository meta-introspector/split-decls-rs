// Generated macro for impl_6987 (impl)
macro_rules! Depcrate_features_gen_StyleSheetChangeEventInitimpl_6987 {
() => {
// Module: crate::features::gen_StyleSheetChangeEventInit
// Provides: {"impl_6987"}
// Dependencies: {}
impl StyleSheetChangeEventInit { # [doc = "Construct a new `StyleSheetChangeEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_document_sheet()` instead."] pub fn document_sheet (& mut self , val : bool) -> & mut Self { self . set_document_sheet (val) ; self } # [cfg (feature = "CssStyleSheet")] # [deprecated = "Use `set_stylesheet()` instead."] pub fn stylesheet (& mut self , val : Option < & CssStyleSheet >) -> & mut Self { self . set_stylesheet (val) ; self } }
};
}
