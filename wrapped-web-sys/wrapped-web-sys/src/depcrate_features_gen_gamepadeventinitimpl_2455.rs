// Generated macro for impl_2455 (impl)
macro_rules! Depcrate_features_gen_GamepadEventInitimpl_2455 {
() => {
// Module: crate::features::gen_GamepadEventInit
// Provides: {"impl_2455"}
// Dependencies: {}
impl GamepadEventInit { # [doc = "Construct a new `GamepadEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GamepadEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "Gamepad")] # [deprecated = "Use `set_gamepad()` instead."] pub fn gamepad (& mut self , val : Option < & Gamepad >) -> & mut Self { self . set_gamepad (val) ; self } }
};
}
