// Generated macro for impl_4564 (impl)
macro_rules! Depcrate_features_gen_MediaStreamEventInitimpl_4564 {
() => {
// Module: crate::features::gen_MediaStreamEventInit
// Provides: {"impl_4564"}
// Dependencies: {}
impl MediaStreamEventInit { # [doc = "Construct a new `MediaStreamEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "MediaStream")] # [deprecated = "Use `set_stream()` instead."] pub fn stream (& mut self , val : Option < & MediaStream >) -> & mut Self { self . set_stream (val) ; self } }
};
}
