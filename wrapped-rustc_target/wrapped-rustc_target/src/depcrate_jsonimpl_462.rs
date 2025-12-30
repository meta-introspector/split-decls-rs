// Generated macro for impl_462 (impl)
macro_rules! Depcrate_jsonimpl_462 {
() => {
// Module: crate::json
// Provides: {"impl_462"}
// Dependencies: {}
impl < 'a , A : ToJson > ToJson for Cow < 'a , [A] > where [A] : ToOwned , { fn to_json (& self) -> Json { Json :: Array (self . iter () . map (| elt | elt . to_json ()) . collect ()) } }
};
}
