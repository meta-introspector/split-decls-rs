// Generated macro for impl_7871 (impl)
macro_rules! Depcrate_features_gen_TrackEventInitimpl_7871 {
() => {
// Module: crate::features::gen_TrackEventInit
// Provides: {"impl_7871"}
// Dependencies: {}
impl TrackEventInit { # [doc = "Construct a new `TrackEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_track()` instead."] pub fn track (& mut self , val : Option < & :: js_sys :: Object >) -> & mut Self { self . set_track (val) ; self } }
};
}
