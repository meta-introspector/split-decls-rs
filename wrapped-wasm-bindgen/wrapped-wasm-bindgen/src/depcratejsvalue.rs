// Generated macro for JsValue (struct)
macro_rules! DepcrateJsValue {
() => {
// Module: crate
// Provides: {"JsValue"}
// Dependencies: {}
# [doc = " Representation of an object owned by JS."] # [doc = ""] # [doc = " A `JsValue` doesn't actually live in Rust right now but actually in a table"] # [doc = " owned by the `wasm-bindgen` generated JS glue code. Eventually the ownership"] # [doc = " will transfer into Wasm directly and this will likely become more efficient,"] # [doc = " but for now it may be slightly slow."] pub struct JsValue { idx : u32 , _marker : PhantomData < * mut u8 > , }
};
}
