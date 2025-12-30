// Generated macro for impl_328 (impl)
macro_rules! Depcrateimpl_328 {
() => {
// Module: crate
// Provides: {"impl_328"}
// Dependencies: {}
impl TryFromJsValue for isize { # [inline] fn try_from_js_value_ref (val : & JsValue) -> Option < isize > { val . as_f64 () . map (| n | n as isize) } }
};
}
