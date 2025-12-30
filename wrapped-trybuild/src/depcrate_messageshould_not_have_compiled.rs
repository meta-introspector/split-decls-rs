// Generated macro for should_not_have_compiled (function)
macro_rules! Depcrate_messageshould_not_have_compiled {
() => {
// Module: crate::message
// Provides: {"should_not_have_compiled"}
// Dependencies: {}
pub (crate) fn should_not_have_compiled () { term :: bold_color (Red) ; println ! ("error") ; term :: color (Red) ; println ! ("Expected test case to fail to compile, but it succeeded.") ; term :: reset () ; println ! () ; }
};
}
