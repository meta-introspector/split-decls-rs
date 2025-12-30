// Generated macro for impl_5399 (impl)
macro_rules! Depcrate_features_gen_PopupBlockedEventInitimpl_5399 {
() => {
// Module: crate::features::gen_PopupBlockedEventInit
// Provides: {"impl_5399"}
// Dependencies: {}
impl PopupBlockedEventInit { # [doc = "Construct a new `PopupBlockedEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_popup_window_features()` instead."] pub fn popup_window_features (& mut self , val : & str) -> & mut Self { self . set_popup_window_features (val) ; self } # [deprecated = "Use `set_popup_window_name()` instead."] pub fn popup_window_name (& mut self , val : & str) -> & mut Self { self . set_popup_window_name (val) ; self } # [cfg (feature = "Window")] # [deprecated = "Use `set_requesting_window()` instead."] pub fn requesting_window (& mut self , val : Option < & Window >) -> & mut Self { self . set_requesting_window (val) ; self } }
};
}
