// Generated macro for is_attributes_extendable (function)
macro_rules! Depcrate_utilsis_attributes_extendable {
() => {
// Module: crate::utils
// Provides: {"is_attributes_extendable"}
// Dependencies: {}
# [inline] pub (crate) fn is_attributes_extendable (attrs_str : & str) -> bool { ! attrs_str . contains ('\n') && ! last_line_contains_single_line_comment (attrs_str) }
};
}
