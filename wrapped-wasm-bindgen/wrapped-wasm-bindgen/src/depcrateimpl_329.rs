// Generated macro for impl_329 (impl)
macro_rules! Depcrateimpl_329 {
() => {
// Module: crate
// Provides: {"impl_329"}
// Dependencies: {}
impl TryFromJsValue for usize { # [inline] fn try_from_js_value_ref (val : & JsValue) -> Option < usize > { val . as_f64 () . map (| n | n as usize) } }
};
}
