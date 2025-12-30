// Generated macro for impl_5102 (impl)
macro_rules! Depcrate_features_gen_PageTransitionEventInitimpl_5102 {
() => {
// Module: crate::features::gen_PageTransitionEventInit
// Provides: {"impl_5102"}
// Dependencies: {}
impl PageTransitionEventInit { # [doc = "Construct a new `PageTransitionEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_in_frame_swap()` instead."] pub fn in_frame_swap (& mut self , val : bool) -> & mut Self { self . set_in_frame_swap (val) ; self } # [deprecated = "Use `set_persisted()` instead."] pub fn persisted (& mut self , val : bool) -> & mut Self { self . set_persisted (val) ; self } }
};
}
