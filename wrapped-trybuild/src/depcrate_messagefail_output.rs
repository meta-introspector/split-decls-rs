// Generated macro for fail_output (function)
macro_rules! Depcrate_messagefail_output {
() => {
// Module: crate::message
// Provides: {"fail_output"}
// Dependencies: {}
pub (crate) fn fail_output (level : Level , stdout : & str) { let color = match level { Fail => Red , Warn => Yellow , } ; if ! stdout . is_empty () { term :: bold_color (color) ; println ! ("STDOUT:") ; snippet (color , & normalize :: trim (stdout)) ; println ! () ; } }
};
}
