// Generated macro for unwrap_function (function)
macro_rules! Depcrateunwrap_function {
() => {
// Module: crate
// Provides: {"unwrap_function"}
// Dependencies: {}
pub fn unwrap_function (struct_name : & str) -> String { let mut name = "__wbg_" . to_string () ; name . extend (struct_name . chars () . flat_map (| s | s . to_lowercase ())) ; name . push_str ("_unwrap") ; name }
};
}
