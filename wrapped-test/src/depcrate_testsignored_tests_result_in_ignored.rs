// Generated macro for ignored_tests_result_in_ignored (function)
macro_rules! Depcrate_testsignored_tests_result_in_ignored {
() => {
// Module: crate::tests
// Provides: {"ignored_tests_result_in_ignored"}
// Dependencies: {}
# [test] fn ignored_tests_result_in_ignored () { fn f () -> Result < () , String > { Ok (()) } let desc = TestDescAndFn { desc : TestDesc { name : StaticTestName ("whatever") , ignore : true , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic : ShouldPanic :: No , compile_fail : false , no_run : false , test_type : TestType :: Unknown , } , testfn : DynTestFn (Box :: new (f)) , } ; let (tx , rx) = channel () ; run_test (& TestOpts :: new () , false , TestId (0) , desc , RunStrategy :: InProcess , tx) ; let result = rx . recv () . unwrap () . result ; assert_eq ! (result , TrIgnored) ; }
};
}
