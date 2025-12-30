// Generated macro for test_fail (function)
macro_rules! Depcrate_messagetest_fail {
() => {
// Module: crate::message
// Provides: {"test_fail"}
// Dependencies: {}
pub (crate) fn test_fail (err : Error) { if err . already_printed () { return ; } term :: bold_color (Red) ; println ! ("error") ; term :: color (Red) ; println ! ("{}" , err) ; term :: reset () ; println ! () ; }
};
}
