// Generated macro for begin_test (function)
macro_rules! Depcrate_messagebegin_test {
() => {
// Module: crate::message
// Provides: {"begin_test"}
// Dependencies: {}
pub (crate) fn begin_test (test : & Test , show_expected : bool) { let display_name = test . path . as_os_str () . to_string_lossy () ; print ! ("test ") ; term :: bold () ; print ! ("{}" , display_name) ; term :: reset () ; if show_expected { match test . expected { Expected :: Pass => print ! (" [should pass]") , Expected :: CompileFail => print ! (" [should fail to compile]") , } } print ! (" ... ") ; }
};
}
