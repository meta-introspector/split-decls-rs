// Generated macro for impl_1652 (impl)
macro_rules! Depcrate_features_gen_DocumentTimelineOptionsimpl_1652 {
() => {
// Module: crate::features::gen_DocumentTimelineOptions
// Provides: {"impl_1652"}
// Dependencies: {}
impl DocumentTimelineOptions { # [doc = "Construct a new `DocumentTimelineOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DocumentTimelineOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_origin_time()` instead."] pub fn origin_time (& mut self , val : f64) -> & mut Self { self . set_origin_time (val) ; self } }
};
}
