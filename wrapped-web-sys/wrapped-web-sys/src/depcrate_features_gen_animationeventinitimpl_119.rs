// Generated macro for impl_119 (impl)
macro_rules! Depcrate_features_gen_AnimationEventInitimpl_119 {
() => {
// Module: crate::features::gen_AnimationEventInit
// Provides: {"impl_119"}
// Dependencies: {}
impl AnimationEventInit { # [doc = "Construct a new `AnimationEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_animation_name()` instead."] pub fn animation_name (& mut self , val : & str) -> & mut Self { self . set_animation_name (val) ; self } # [deprecated = "Use `set_elapsed_time()` instead."] pub fn elapsed_time (& mut self , val : f32) -> & mut Self { self . set_elapsed_time (val) ; self } # [deprecated = "Use `set_pseudo_element()` instead."] pub fn pseudo_element (& mut self , val : & str) -> & mut Self { self . set_pseudo_element (val) ; self } }
};
}
