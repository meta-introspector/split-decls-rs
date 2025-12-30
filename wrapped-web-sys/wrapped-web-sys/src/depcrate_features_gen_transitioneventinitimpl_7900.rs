// Generated macro for impl_7900 (impl)
macro_rules! Depcrate_features_gen_TransitionEventInitimpl_7900 {
() => {
// Module: crate::features::gen_TransitionEventInit
// Provides: {"impl_7900"}
// Dependencies: {}
impl TransitionEventInit { # [doc = "Construct a new `TransitionEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_elapsed_time()` instead."] pub fn elapsed_time (& mut self , val : f32) -> & mut Self { self . set_elapsed_time (val) ; self } # [deprecated = "Use `set_property_name()` instead."] pub fn property_name (& mut self , val : & str) -> & mut Self { self . set_property_name (val) ; self } # [deprecated = "Use `set_pseudo_element()` instead."] pub fn pseudo_element (& mut self , val : & str) -> & mut Self { self . set_pseudo_element (val) ; self } }
};
}
