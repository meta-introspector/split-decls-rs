// Generated macro for free_function (function)
macro_rules! Depcratefree_function {
() => {
// Module: crate
// Provides: {"free_function"}
// Dependencies: {}
pub fn free_function (struct_name : & str) -> String { let mut name = "__wbg_" . to_string () ; name . extend (struct_name . chars () . flat_map (| s | s . to_lowercase ())) ; name . push_str ("_free") ; name }
};
}
