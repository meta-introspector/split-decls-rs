// Generated macro for test_lint (function)
macro_rules! Depcratetest_lint {
() => {
// Module: crate
// Provides: {"test_lint"}
// Dependencies: {}
fn test_lint () { println ! ("🧪 lint") ; for cwd in [None , Some ("defmt-03/") , Some ("firmware/")] { do_test (| | run_command ("cargo" , & ["fmt" , "--" , "--check"] , cwd , & []) , "lint" ,) ; } do_test (| | run_command ("cargo" , & ["clippy" , "--" , "-D" , "warnings"] , None , & []) , "lint" ,) ; }
};
}
