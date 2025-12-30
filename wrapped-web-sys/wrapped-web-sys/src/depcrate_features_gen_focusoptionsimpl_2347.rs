// Generated macro for impl_2347 (impl)
macro_rules! Depcrate_features_gen_FocusOptionsimpl_2347 {
() => {
// Module: crate::features::gen_FocusOptions
// Provides: {"impl_2347"}
// Dependencies: {}
impl FocusOptions { # [doc = "Construct a new `FocusOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FocusOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_focus_visible()` instead."] pub fn focus_visible (& mut self , val : bool) -> & mut Self { self . set_focus_visible (val) ; self } # [deprecated = "Use `set_prevent_scroll()` instead."] pub fn prevent_scroll (& mut self , val : bool) -> & mut Self { self . set_prevent_scroll (val) ; self } }
};
}
