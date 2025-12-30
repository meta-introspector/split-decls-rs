// Generated macro for WasmClosureFnOnce (trait)
macro_rules! Depcrate_closureWasmClosureFnOnce {
() => {
// Module: crate::closure
// Provides: {"WasmClosureFnOnce"}
// Dependencies: {}
# [doc = " A trait for converting an `FnOnce(A...) -> R` into a `FnMut(A...) -> R` that"] # [doc = " will throw if ever called more than once."] # [doc (hidden)] pub trait WasmClosureFnOnce < FnMut : ? Sized , A , R > : 'static { fn into_fn_mut (self) -> Box < FnMut > ; fn into_js_function (self) -> JsValue ; }
};
}
