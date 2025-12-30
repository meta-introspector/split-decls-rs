// Generated macro for sample_tests (function)
macro_rules! Depcrate_testssample_tests {
() => {
// Module: crate::tests
// Provides: {"sample_tests"}
// Dependencies: {}
fn sample_tests () -> Vec < TestDescAndFn > { let names = vec ! ["sha1::test" . to_string () , "isize::test_to_str" . to_string () , "isize::test_pow" . to_string () , "test::do_not_run_ignored_tests" . to_string () , "test::ignored_tests_result_in_ignored" . to_string () , "test::first_free_arg_should_be_a_filter" . to_string () , "test::parse_ignored_flag" . to_string () , "test::parse_include_ignored_flag" . to_string () , "test::filter_for_ignored_option" . to_string () , "test::run_include_ignored_option" . to_string () , "test::sort_tests" . to_string () ,] ; fn testfn () -> Result < () , String > { Ok (()) } let mut tests = Vec :: new () ; for name in & names { let test = TestDescAndFn { desc : TestDesc { name : DynTestName ((* name) . clone ()) , ignore : false , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic : ShouldPanic :: No , compile_fail : false , no_run : false , test_type : TestType :: Unknown , } , testfn : DynTestFn (Box :: new (testfn)) , } ; tests . push (test) ; } tests }
};
}
