// Generated macro for impl_972 (impl)
macro_rules! Depcrate_features_gen_ClipboardItemOptionsimpl_972 {
() => {
// Module: crate::features::gen_ClipboardItemOptions
// Provides: {"impl_972"}
// Dependencies: {}
impl ClipboardItemOptions { # [doc = "Construct a new `ClipboardItemOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ClipboardItemOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "PresentationStyle")] # [deprecated = "Use `set_presentation_style()` instead."] pub fn presentation_style (& mut self , val : PresentationStyle) -> & mut Self { self . set_presentation_style (val) ; self } }
};
}
