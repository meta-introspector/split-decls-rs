// Generated macro for contains_problematic_const (function)
macro_rules! Depcrate_stylecontains_problematic_const {
() => {
// Module: crate::style
// Provides: {"contains_problematic_const"}
// Dependencies: {}
fn contains_problematic_const (trimmed : & str) -> bool { PROBLEMATIC_CONSTS_STRINGS . iter () . any (| s | trimmed . to_uppercase () . contains (s)) }
};
}
