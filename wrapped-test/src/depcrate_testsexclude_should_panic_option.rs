// Generated macro for exclude_should_panic_option (function)
macro_rules! Depcrate_testsexclude_should_panic_option {
() => {
// Module: crate::tests
// Provides: {"exclude_should_panic_option"}
// Dependencies: {}
# [test] fn exclude_should_panic_option () { let mut opts = TestOpts :: new () ; opts . run_tests = true ; opts . exclude_should_panic = true ; let mut tests = one_ignored_one_unignored_test () ; tests . push (TestDescAndFn { desc : TestDesc { name : StaticTestName ("3") , ignore : false , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic : ShouldPanic :: Yes , compile_fail : false , no_run : false , test_type : TestType :: Unknown , } , testfn : DynTestFn (Box :: new (move | | Ok (()))) , }) ; let filtered = filter_tests (& opts , tests) ; assert_eq ! (filtered . len () , 2) ; assert ! (filtered . iter () . all (| test | test . desc . should_panic == ShouldPanic :: No)) ; }
};
}
