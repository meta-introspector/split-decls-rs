// Generated macro for impl_769 (impl)
macro_rules! Depcrate_specimpl_769 {
() => {
// Module: crate::spec
// Provides: {"impl_769"}
// Dependencies: {}
impl ToJson for SanitizerSet { fn to_json (& self) -> Json { self . into_iter () . map (| v | Some (v . as_str () ? . to_json ())) . collect :: < Option < Vec < _ > > > () . unwrap_or_default () . to_json () } }
};
}
