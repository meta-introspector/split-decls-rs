// Generated macro for JsThreadLocal (struct)
macro_rules! DepcrateJsThreadLocal {
() => {
// Module: crate
// Provides: {"JsThreadLocal"}
// Dependencies: {}
# [doc = " Wrapper type for imported statics."] # [doc = ""] # [doc = " This type is used whenever a `static` is imported from a JS module, for"] # [doc = " example this import:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[wasm_bindgen]"] # [doc = " extern \"C\" {"] # [doc = "     #[wasm_bindgen(thread_local_v2)]"] # [doc = "     static console: JsValue;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " will generate in Rust a value that looks like:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " static console: JsThreadLocal<JsValue> = ...;"] # [doc = " ```"] pub struct JsThreadLocal < T : 'static > { # [doc (hidden)] # [cfg (not (target_feature = "atomics"))] pub __inner : & 'static __rt :: LazyCell < T > , # [doc (hidden)] # [cfg (target_feature = "atomics")] pub __inner : fn () -> * const T , }
};
}
