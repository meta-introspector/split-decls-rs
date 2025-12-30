// Generated macro for parse_class_name_integration (function)
macro_rules! Depcrate_formatters_junitparse_class_name_integration {
() => {
// Module: crate::formatters::junit
// Provides: {"parse_class_name_integration"}
// Dependencies: {}
fn parse_class_name_integration (desc : & TestDesc) -> (String , String) { (String :: from ("integration") , String :: from (desc . name . as_slice ())) }
};
}
