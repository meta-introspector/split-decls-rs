// Generated macro for UncheckedIter (struct)
macro_rules! Depcrate_iterUncheckedIter {
() => {
// Module: crate::iter
// Provides: {"UncheckedIter"}
// Dependencies: {}
# [doc = " A wrapper around JS Iterator so it can be consumed from Rust."] # [doc = ""] # [doc = " This type implements [`Iterator`] trait and will keep yielding [`JsValue`]"] # [doc = " until the underlying [`js_sys::Iterator`] is exuasted."] # [doc = ""] # [doc = " This type is called `UncheckedIter` because it does no checking for"] # [doc = " the underlying type of the [`js_sys::Iterator`] and yields [`JsValue`]s."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use gloo_utils::iter::UncheckedIter;"] # [doc = " use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};"] # [doc = ""] # [doc = " # fn no_run() {"] # [doc = " let map = js_sys::Map::new();"] # [doc = " map.set(&JsValue::from(\"one\"), &JsValue::from(1_f64));"] # [doc = ""] # [doc = " let mut iter = UncheckedIter::from(map.entries()).map(|js_value| {"] # [doc = "     let array: js_sys::Array = js_value.unchecked_into();"] # [doc = "     ("] # [doc = "         array.get(0).as_string().unwrap_throw(),"] # [doc = "         array.get(1).as_f64().unwrap_throw(),"] # [doc = "     )"] # [doc = " });"] # [doc = ""] # [doc = " assert_eq!(iter.next(), Some((String::from(\"one\"), 1_f64)));"] # [doc = " assert_eq!(iter.next(), None);"] # [doc = " # }"] # [doc = " ```"] pub struct UncheckedIter (js_sys :: Iterator) ;
};
}
