// Generated macro for impl_71 (impl)
macro_rules! Depcrate_typescript_typeimpl_71 {
() => {
// Module: crate::typescript_type
// Provides: {"impl_71"}
// Dependencies: {}
# [wasm_bindgen] impl TextStyle { # [wasm_bindgen (constructor)] pub fn new (i : ITextStyle) -> TextStyle { let js_value : JsValue = i . into () ; serde_wasm_bindgen :: from_value (js_value) . unwrap () } pub fn optional_new (i : Option < ITextStyle >) -> TextStyle { i . map (Self :: new) . unwrap_or_default () } }
};
}
