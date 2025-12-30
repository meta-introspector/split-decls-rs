// Generated macro for impl_730 (impl)
macro_rules! Depcrate_specimpl_730 {
() => {
// Module: crate::spec
// Provides: {"impl_730"}
// Dependencies: {}
impl ToJson for LinkSelfContainedComponents { fn to_json (& self) -> Json { let components : Vec < _ > = Self :: all_components () . into_iter () . filter (| c | self . contains (* c)) . map (| c | { c . as_str () . unwrap () . to_owned () }) . collect () ; components . to_json () } }
};
}
