// Generated macro for new_function (function)
macro_rules! Depcratenew_function {
() => {
// Module: crate
// Provides: {"new_function"}
// Dependencies: {}
pub fn new_function (struct_name : & str) -> String { let mut name = "__wbg_" . to_string () ; name . extend (struct_name . chars () . flat_map (| s | s . to_lowercase ())) ; name . push_str ("_new") ; name }
};
}
