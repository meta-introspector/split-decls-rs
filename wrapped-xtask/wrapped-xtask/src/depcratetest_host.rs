// Generated macro for test_host (function)
macro_rules! Depcratetest_host {
() => {
// Module: crate
// Provides: {"test_host"}
// Dependencies: {}
fn test_host (deny_warnings : bool , skip_ui_tests : bool) { println ! ("🧪 host") ; let mut env = vec ! [] ; if skip_ui_tests { env . push (("SKIP_UI_TESTS" , "1")) ; } if deny_warnings { env . push (("RUSTFLAGS" , "--deny warnings")) ; } for feat in ["" , "unstable-test" , "alloc"] { do_test (| | run_command ("cargo" , & ["check" , "--features" , feat] , None , & env) , "host" ,) ; } for feat in ["unstable-test" , "unstable-test,alloc"] { do_test (| | run_command ("cargo" , & ["test" , "--features" , feat] , None , & env) , "host" ,) ; } }
};
}
