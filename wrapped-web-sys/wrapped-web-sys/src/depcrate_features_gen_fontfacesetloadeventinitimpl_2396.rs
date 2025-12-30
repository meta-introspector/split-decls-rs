// Generated macro for impl_2396 (impl)
macro_rules! Depcrate_features_gen_FontFaceSetLoadEventInitimpl_2396 {
() => {
// Module: crate::features::gen_FontFaceSetLoadEventInit
// Provides: {"impl_2396"}
// Dependencies: {}
impl FontFaceSetLoadEventInit { # [doc = "Construct a new `FontFaceSetLoadEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [deprecated = "Use `set_fontfaces()` instead."] pub fn fontfaces (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_fontfaces (val) ; self } }
};
}
