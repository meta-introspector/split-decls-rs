// Generated macro for impl_15 (impl)
macro_rules! Depcrate_field_debugimpl_15 {
() => {
// Module: crate::field::debug
// Provides: {"impl_15"}
// Dependencies: {}
impl < V > Alt < V > { # [doc = " Wraps the provided visitor so that any `fmt::Debug` fields are formatted"] # [doc = " using the alternative (`:#`) formatter."] pub fn new (inner : V) -> Self { Alt (inner) } }
};
}
