// Generated macro for JsStatic (struct)
macro_rules! DepcrateJsStatic {
() => {
// Module: crate
// Provides: {"JsStatic"}
// Dependencies: {}
# [doc = " Wrapper type for imported statics."] # [doc = ""] # [doc = " This type is used whenever a `static` is imported from a JS module, for"] # [doc = " example this import:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[wasm_bindgen]"] # [doc = " extern \"C\" {"] # [doc = "     static console: JsValue;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " will generate in Rust a value that looks like:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " static console: JsStatic<JsValue> = ...;"] # [doc = " ```"] # [doc = ""] # [doc = " This type implements `Deref` to the inner type so it's typically used as if"] # [doc = " it were `&T`."] # [cfg (feature = "std")] # [deprecated = "use with `#[wasm_bindgen(thread_local_v2)]` instead"] pub struct JsStatic < T : 'static > { # [doc (hidden)] pub __inner : & 'static std :: thread :: LocalKey < T > , }
};
}
