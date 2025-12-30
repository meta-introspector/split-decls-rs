// Generated macro for impl_135 (impl)
macro_rules! Depcrate_features_gen_AnimationPlaybackEventInitimpl_135 {
() => {
// Module: crate::features::gen_AnimationPlaybackEventInit
// Provides: {"impl_135"}
// Dependencies: {}
impl AnimationPlaybackEventInit { # [doc = "Construct a new `AnimationPlaybackEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_current_time()` instead."] pub fn current_time (& mut self , val : Option < f64 >) -> & mut Self { self . set_current_time (val) ; self } # [deprecated = "Use `set_timeline_time()` instead."] pub fn timeline_time (& mut self , val : Option < f64 >) -> & mut Self { self . set_timeline_time (val) ; self } }
};
}
