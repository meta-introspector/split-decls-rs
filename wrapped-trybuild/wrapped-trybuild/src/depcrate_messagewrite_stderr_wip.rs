// Generated macro for write_stderr_wip (function)
macro_rules! Depcrate_messagewrite_stderr_wip {
() => {
// Module: crate::message
// Provides: {"write_stderr_wip"}
// Dependencies: {}
pub (crate) fn write_stderr_wip (wip_path : & Path , stderr_path : & Path , stderr : & str) { let wip_path = wip_path . to_string_lossy () ; let stderr_path = stderr_path . to_string_lossy () ; term :: bold_color (Yellow) ; println ! ("wip") ; println ! () ; print ! ("NOTE") ; term :: reset () ; println ! (": writing the following output to `{}`." , wip_path) ; println ! ("Move this file to `{}` to accept it as correct." , stderr_path ,) ; snippet (Yellow , stderr) ; println ! () ; }
};
}
