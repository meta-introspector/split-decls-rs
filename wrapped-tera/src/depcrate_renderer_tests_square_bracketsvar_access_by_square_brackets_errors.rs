// Generated macro for var_access_by_square_brackets_errors (function)
macro_rules! Depcrate_renderer_tests_square_bracketsvar_access_by_square_brackets_errors {
() => {
// Module: crate::renderer::tests::square_brackets
// Provides: {"var_access_by_square_brackets_errors"}
// Dependencies: {}
# [test] fn var_access_by_square_brackets_errors () { let mut context = Context :: new () ; context . insert ("var" , & Test { a : "hi" . into () , b : "there" . into () , c : vec ! [] }) ; let t = Tera :: one_off ("{{var[csd]}}" , & context , true) ; assert ! (t . is_err () , "Access of csd should be impossible") ; }
};
}
