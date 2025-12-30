// Generated macro for parse_class_name_unit (function)
macro_rules! Depcrate_formatters_junitparse_class_name_unit {
() => {
// Module: crate::formatters::junit
// Provides: {"parse_class_name_unit"}
// Dependencies: {}
fn parse_class_name_unit (desc : & TestDesc) -> (String , String) { let module_segments : Vec < & str > = desc . name . as_slice () . split ("::") . collect () ; let (class_name , test_name) = match module_segments [..] { [test] => (String :: from ("crate") , String :: from (test)) , [ref path @ .. , test] => (path . join ("::") , String :: from (test)) , [..] => unreachable ! () , } ; (class_name , test_name) }
};
}
