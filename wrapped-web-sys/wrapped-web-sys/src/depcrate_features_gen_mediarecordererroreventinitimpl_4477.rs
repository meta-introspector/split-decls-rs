// Generated macro for impl_4477 (impl)
macro_rules! Depcrate_features_gen_MediaRecorderErrorEventInitimpl_4477 {
() => {
// Module: crate::features::gen_MediaRecorderErrorEventInit
// Provides: {"impl_4477"}
// Dependencies: {}
impl MediaRecorderErrorEventInit { # [cfg (feature = "DomException")] # [doc = "Construct a new `MediaRecorderErrorEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DomException`, `MediaRecorderErrorEventInit`*"] pub fn new (error : & DomException) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_error (error) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "DomException")] # [deprecated = "Use `set_error()` instead."] pub fn error (& mut self , val : & DomException) -> & mut Self { self . set_error (val) ; self } }
};
}
