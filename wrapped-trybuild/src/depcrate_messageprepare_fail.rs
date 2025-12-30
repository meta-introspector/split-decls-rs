// Generated macro for prepare_fail (function)
macro_rules! Depcrate_messageprepare_fail {
() => {
// Module: crate::message
// Provides: {"prepare_fail"}
// Dependencies: {}
pub (crate) fn prepare_fail (err : Error) { if err . already_printed () { return ; } term :: bold_color (Red) ; print ! ("ERROR") ; term :: reset () ; println ! (": {}" , err) ; println ! () ; }
};
}
