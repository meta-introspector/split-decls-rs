// Generated macro for parse_class_name_doc (function)
macro_rules! Depcrate_formatters_junitparse_class_name_doc {
() => {
// Module: crate::formatters::junit
// Provides: {"parse_class_name_doc"}
// Dependencies: {}
fn parse_class_name_doc (desc : & TestDesc) -> (String , String) { let segments : Vec < & str > = desc . name . as_slice () . split (" - ") . collect () ; let (class_name , test_name) = match segments [..] { [file , line] => (String :: from (file . trim ()) , String :: from (line . trim ())) , [..] => unreachable ! () , } ; (class_name , test_name) }
};
}
