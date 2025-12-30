// Generated macro for must_contain (function)
macro_rules! Depcrate__privatemust_contain {
() => {
// Module: crate::_private
// Provides: {"must_contain"}
// Dependencies: {}
pub fn must_contain (schema : & mut Schema , substring : & str) { let escaped = regex_syntax :: escape (substring) ; insert_validation_property (schema , "string" , "pattern" , escaped) ; }
};
}
