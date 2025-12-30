// Generated macro for impl_6846 (impl)
macro_rules! Depcrate_features_gen_SpeechRecognitionErrorInitimpl_6846 {
() => {
// Module: crate::features::gen_SpeechRecognitionErrorInit
// Provides: {"impl_6846"}
// Dependencies: {}
impl SpeechRecognitionErrorInit { # [doc = "Construct a new `SpeechRecognitionErrorInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "SpeechRecognitionErrorCode")] # [deprecated = "Use `set_error()` instead."] pub fn error (& mut self , val : SpeechRecognitionErrorCode) -> & mut Self { self . set_error (val) ; self } # [deprecated = "Use `set_message()` instead."] pub fn message (& mut self , val : & str) -> & mut Self { self . set_message (val) ; self } }
};
}
