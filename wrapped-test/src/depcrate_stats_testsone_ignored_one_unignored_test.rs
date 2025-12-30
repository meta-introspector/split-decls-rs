// Generated macro for one_ignored_one_unignored_test (function)
macro_rules! Depcrate_stats_testsone_ignored_one_unignored_test {
() => {
// Module: crate::stats::tests
// Provides: {"one_ignored_one_unignored_test"}
// Dependencies: {}
fn one_ignored_one_unignored_test () -> Vec < TestDescAndFn > { vec ! [TestDescAndFn { desc : TestDesc { name : StaticTestName ("1") , ignore : true , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic : ShouldPanic :: No , compile_fail : false , no_run : false , test_type : TestType :: Unknown , } , testfn : DynTestFn (Box :: new (move || Ok (()))) , } , TestDescAndFn { desc : TestDesc { name : StaticTestName ("2") , ignore : false , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic : ShouldPanic :: No , compile_fail : false , no_run : false , test_type : TestType :: Unknown , } , testfn : DynTestFn (Box :: new (move || Ok (()))) , } ,] }
};
}
