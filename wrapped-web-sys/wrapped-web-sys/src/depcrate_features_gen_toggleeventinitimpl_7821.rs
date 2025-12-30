// Generated macro for impl_7821 (impl)
macro_rules! Depcrate_features_gen_ToggleEventInitimpl_7821 {
() => {
// Module: crate::features::gen_ToggleEventInit
// Provides: {"impl_7821"}
// Dependencies: {}
impl ToggleEventInit { # [doc = "Construct a new `ToggleEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_new_state()` instead."] pub fn new_state (& mut self , val : & str) -> & mut Self { self . set_new_state (val) ; self } # [deprecated = "Use `set_old_state()` instead."] pub fn old_state (& mut self , val : & str) -> & mut Self { self . set_old_state (val) ; self } }
};
}
