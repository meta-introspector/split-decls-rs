// Generated macro for stdin_works_with_checkstyle (function)
macro_rules! Depcrate_teststdin_works_with_checkstyle {
() => {
// Module: crate::test
// Provides: {"stdin_works_with_checkstyle"}
// Dependencies: {}
# [doc = " Ensures that `EmitMode::Checkstyle` works with input from `stdin`."] # [test] fn stdin_works_with_checkstyle () { init_log () ; assert_stdin_output (Path :: new ("tests/writemode/source/stdin.rs") , Path :: new ("tests/writemode/target/stdin.xml") , EmitMode :: Checkstyle , false ,) ; }
};
}
