// Generated macro for impl_3981 (impl)
macro_rules! Depcrate_features_gen_ImageCaptureErrorEventInitimpl_3981 {
() => {
// Module: crate::features::gen_ImageCaptureErrorEventInit
// Provides: {"impl_3981"}
// Dependencies: {}
impl ImageCaptureErrorEventInit { # [doc = "Construct a new `ImageCaptureErrorEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "ImageCaptureError")] # [deprecated = "Use `set_image_capture_error()` instead."] pub fn image_capture_error (& mut self , val : Option < & ImageCaptureError >) -> & mut Self { self . set_image_capture_error (val) ; self } }
};
}
