// Generated macro for test_lint_cross (function)
macro_rules! Depcratetest_lint_cross {
() => {
// Module: crate
// Provides: {"test_lint_cross"}
// Dependencies: {}
fn test_lint_cross (deny_warnings : bool) { println ! ("🧪 lint-cross") ; let env = match deny_warnings { true => vec ! [("RUSTFLAGS" , "--deny warnings")] , false => vec ! [] , } ; do_test (| | { run_command ("cargo" , & ["clippy" , "--target" , "thumbv7m-none-eabi" , "--" , "-D" , "warnings" , "-A" , "unknown-lints" ,] , Some ("firmware/") , & env ,) } , "cross" ,) ; }
};
}
