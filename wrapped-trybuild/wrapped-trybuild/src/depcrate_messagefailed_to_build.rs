// Generated macro for failed_to_build (function)
macro_rules! Depcrate_messagefailed_to_build {
() => {
// Module: crate::message
// Provides: {"failed_to_build"}
// Dependencies: {}
pub (crate) fn failed_to_build (stderr : & str) { term :: bold_color (Red) ; println ! ("error") ; snippet (Red , stderr) ; println ! () ; }
};
}
