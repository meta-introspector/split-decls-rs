// Generated macro for impl_6858 (impl)
macro_rules! Depcrate_features_gen_SpeechRecognitionEventInitimpl_6858 {
() => {
// Module: crate::features::gen_SpeechRecognitionEventInit
// Provides: {"impl_6858"}
// Dependencies: {}
impl SpeechRecognitionEventInit { # [doc = "Construct a new `SpeechRecognitionEventInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bubbles()` instead."] pub fn bubbles (& mut self , val : bool) -> & mut Self { self . set_bubbles (val) ; self } # [deprecated = "Use `set_cancelable()` instead."] pub fn cancelable (& mut self , val : bool) -> & mut Self { self . set_cancelable (val) ; self } # [deprecated = "Use `set_composed()` instead."] pub fn composed (& mut self , val : bool) -> & mut Self { self . set_composed (val) ; self } # [cfg (feature = "Document")] # [deprecated = "Use `set_emma()` instead."] pub fn emma (& mut self , val : Option < & Document >) -> & mut Self { self . set_emma (val) ; self } # [deprecated = "Use `set_interpretation()` instead."] pub fn interpretation (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_interpretation (val) ; self } # [deprecated = "Use `set_result_index()` instead."] pub fn result_index (& mut self , val : u32) -> & mut Self { self . set_result_index (val) ; self } # [cfg (feature = "SpeechRecognitionResultList")] # [deprecated = "Use `set_results()` instead."] pub fn results (& mut self , val : Option < & SpeechRecognitionResultList >) -> & mut Self { self . set_results (val) ; self } }
};
}
