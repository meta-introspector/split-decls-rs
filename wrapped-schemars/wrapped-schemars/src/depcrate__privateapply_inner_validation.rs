// Generated macro for apply_inner_validation (function)
macro_rules! Depcrate__privateapply_inner_validation {
() => {
// Module: crate::_private
// Provides: {"apply_inner_validation"}
// Dependencies: {}
pub fn apply_inner_validation (schema : & mut Schema , f : fn (& mut Schema) -> ()) { if let Some (inner_schema) = schema . get_mut ("items") . and_then (| i | i . try_into () . ok ()) { f (inner_schema) ; } }
};
}
