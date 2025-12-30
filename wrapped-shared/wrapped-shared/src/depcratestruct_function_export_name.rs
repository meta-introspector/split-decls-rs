// Generated macro for struct_function_export_name (function)
macro_rules! Depcratestruct_function_export_name {
() => {
// Module: crate
// Provides: {"struct_function_export_name"}
// Dependencies: {}
pub fn struct_function_export_name (struct_ : & str , f : & str) -> String { let mut name = struct_ . chars () . flat_map (| s | s . to_lowercase ()) . collect :: < String > () ; name . push ('_') ; name . push_str (f) ; name }
};
}
