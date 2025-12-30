// Generated macro for typed_test_desc (function)
macro_rules! Depcrate_teststyped_test_desc {
() => {
// Module: crate::tests
// Provides: {"typed_test_desc"}
// Dependencies: {}
fn typed_test_desc (test_type : TestType) -> TestDesc { TestDesc { name : StaticTestName ("whatever") , ignore : false , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic : ShouldPanic :: No , compile_fail : false , no_run : false , test_type , } }
};
}
