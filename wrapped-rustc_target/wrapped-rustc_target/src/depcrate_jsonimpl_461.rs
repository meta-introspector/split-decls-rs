// Generated macro for impl_461 (impl)
macro_rules! Depcrate_jsonimpl_461 {
() => {
// Module: crate::json
// Provides: {"impl_461"}
// Dependencies: {}
impl < A : ToJson > ToJson for Vec < A > { fn to_json (& self) -> Json { Json :: Array (self . iter () . map (| elt | elt . to_json ()) . collect ()) } }
};
}
