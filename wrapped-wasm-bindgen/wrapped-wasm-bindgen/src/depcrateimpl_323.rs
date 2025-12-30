// Generated macro for impl_323 (impl)
macro_rules! Depcrateimpl_323 {
() => {
// Module: crate
// Provides: {"impl_323"}
// Dependencies: {}
impl < T : TryFromJsValue > TryFromJsValue for Option < T > { fn try_from_js_value_ref (value : & JsValue) -> Option < Self > { if value . is_undefined () { Some (None) } else { T :: try_from_js_value_ref (value) . map (Some) } } }
};
}
