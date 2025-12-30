// Generated macro for filter_fluent (function)
macro_rules! Depcrate_fluent_lowercasefilter_fluent {
() => {
// Module: crate::fluent_lowercase
// Provides: {"filter_fluent"}
// Dependencies: {}
fn filter_fluent (path : & Path) -> bool { if let Some (ext) = path . extension () { ext . to_str () != Some ("ftl") } else { true } }
};
}
