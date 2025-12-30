// Generated macro for struct_field_set (function)
macro_rules! Depcratestruct_field_set {
() => {
// Module: crate
// Provides: {"struct_field_set"}
// Dependencies: {}
pub fn struct_field_set (struct_ : & str , f : & str) -> String { let mut name = String :: from ("__wbg_set_") ; name . extend (struct_ . chars () . flat_map (| s | s . to_lowercase ())) ; name . push ('_') ; name . push_str (f) ; name }
};
}
