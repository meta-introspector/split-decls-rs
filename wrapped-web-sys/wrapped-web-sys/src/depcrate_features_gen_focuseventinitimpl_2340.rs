// Generated macro for impl_2340 (impl)
macro_rules! Depcrate_features_gen_FocusEventInitimpl_2340 {
() => {
// Module: crate::features::gen_FocusEventInit
// Provides: {"impl_2340"}
// Dependencies: {}
impl FocusEventInit { # [doc = "Construct a new `FocusEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FocusEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_detail()` instead."] pub fn detail (& mut self , val : i32) -> & mut Self { self . set_detail (val) ; self } # [cfg (feature = "Window")] # [deprecated = "Use `set_view()` instead."] pub fn view (& mut self , val : Option < & Window >) -> & mut Self { self . set_view (val) ; self } # [cfg (feature = "EventTarget")] # [deprecated = "Use `set_related_target()` instead."] pub fn related_target (& mut self , val : Option < & EventTarget >) -> & mut Self { self . set_related_target (val) ; self } }
};
}
