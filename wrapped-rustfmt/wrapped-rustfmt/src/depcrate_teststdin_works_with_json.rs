// Generated macro for stdin_works_with_json (function)
macro_rules! Depcrate_teststdin_works_with_json {
() => {
// Module: crate::test
// Provides: {"stdin_works_with_json"}
// Dependencies: {}
# [doc = " Ensures that `EmitMode::Json` works with input from `stdin`."] # [test] fn stdin_works_with_json () { init_log () ; assert_stdin_output (Path :: new ("tests/writemode/source/stdin.rs") , Path :: new ("tests/writemode/target/stdin.json") , EmitMode :: Json , true ,) ; }
};
}
