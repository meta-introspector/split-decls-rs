// Generated macro for struct_field_get (function)
macro_rules! Depcratestruct_field_get {
() => {
// Module: crate
// Provides: {"struct_field_get"}
// Dependencies: {}
pub fn struct_field_get (struct_ : & str , f : & str) -> String { let mut name = String :: from ("__wbg_get_") ; name . extend (struct_ . chars () . flat_map (| s | s . to_lowercase ())) ; name . push ('_') ; name . push_str (f) ; name }
};
}
