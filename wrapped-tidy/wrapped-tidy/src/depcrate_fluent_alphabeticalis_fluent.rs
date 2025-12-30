// Generated macro for is_fluent (function)
macro_rules! Depcrate_fluent_alphabeticalis_fluent {
() => {
// Module: crate::fluent_alphabetical
// Provides: {"is_fluent"}
// Dependencies: {}
fn is_fluent (path : & Path) -> bool { path . extension () . is_some_and (| ext | ext == "flt") }
};
}
