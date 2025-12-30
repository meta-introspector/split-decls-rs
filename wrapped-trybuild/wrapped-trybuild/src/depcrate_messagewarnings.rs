// Generated macro for warnings (function)
macro_rules! Depcrate_messagewarnings {
() => {
// Module: crate::message
// Provides: {"warnings"}
// Dependencies: {}
pub (crate) fn warnings (warnings : & str) { if warnings . is_empty () { return ; } term :: bold_color (Yellow) ; println ! ("WARNINGS:") ; snippet (Yellow , warnings) ; println ! () ; }
};
}
