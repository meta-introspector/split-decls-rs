// Generated macro for impl_4460 (impl)
macro_rules! Depcrate_features_gen_MediaQueryListEventInitimpl_4460 {
() => {
// Module: crate::features::gen_MediaQueryListEventInit
// Provides: {"impl_4460"}
// Dependencies: {}
impl MediaQueryListEventInit { # [doc = "Construct a new `MediaQueryListEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_matches()` instead."] pub fn matches (& mut self , val : bool) -> & mut Self { self . set_matches (val) ; self } # [deprecated = "Use `set_media()` instead."] pub fn media (& mut self , val : & str) -> & mut Self { self . set_media (val) ; self } }
};
}
