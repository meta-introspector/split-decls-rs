// Generated macro for is_type (function)
macro_rules! Depcrate_syn_utilsis_type {
() => {
// Module: crate::syn_utils
// Provides: {"is_type"}
// Dependencies: {}
pub fn is_type (ty : & Type , ns : & [& [& str]] , name : & str) -> bool { if let Some (a) = get_arguments_of (ty , ns , name) { a . is_empty () } else { false } }
};
}
