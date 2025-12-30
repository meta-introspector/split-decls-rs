// Generated macro for impl_307 (impl)
macro_rules! Depcrateimpl_307 {
() => {
// Module: crate
// Provides: {"impl_307"}
// Dependencies: {}
impl TryFromJsValue for char { fn try_from_js_value_ref (value : & JsValue) -> Option < Self > { let s = value . as_string () ? ; if s . len () == 1 { Some (s . chars () . nth (0) . unwrap ()) } else { None } } }
};
}
