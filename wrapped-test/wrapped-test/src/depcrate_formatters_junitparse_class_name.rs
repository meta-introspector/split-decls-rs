// Generated macro for parse_class_name (function)
macro_rules! Depcrate_formatters_junitparse_class_name {
() => {
// Module: crate::formatters::junit
// Provides: {"parse_class_name"}
// Dependencies: {}
fn parse_class_name (desc : & TestDesc) -> (String , String) { match desc . test_type { TestType :: UnitTest => parse_class_name_unit (desc) , TestType :: DocTest => parse_class_name_doc (desc) , TestType :: IntegrationTest => parse_class_name_integration (desc) , TestType :: Unknown => (String :: from ("unknown") , String :: from (desc . name . as_slice ())) , } }
};
}
