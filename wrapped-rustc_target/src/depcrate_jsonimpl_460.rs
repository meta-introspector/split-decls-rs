// Generated macro for impl_460 (impl)
macro_rules! Depcrate_jsonimpl_460 {
() => {
// Module: crate::json
// Provides: {"impl_460"}
// Dependencies: {}
impl < A : ToJson > ToJson for [A] { fn to_json (& self) -> Json { Json :: Array (self . iter () . map (| elt | elt . to_json ()) . collect ()) } }
};
}
