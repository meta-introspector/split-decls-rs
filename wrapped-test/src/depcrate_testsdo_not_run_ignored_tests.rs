// Generated macro for do_not_run_ignored_tests (function)
macro_rules! Depcrate_testsdo_not_run_ignored_tests {
() => {
// Module: crate::tests
// Provides: {"do_not_run_ignored_tests"}
// Dependencies: {}
# [test] fn do_not_run_ignored_tests () { fn f () -> Result < () , String > { panic ! () ; } let desc = TestDescAndFn { desc : TestDesc { name : StaticTestName ("whatever") , ignore : true , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic : ShouldPanic :: No , compile_fail : false , no_run : false , test_type : TestType :: Unknown , } , testfn : DynTestFn (Box :: new (f)) , } ; let (tx , rx) = channel () ; run_test (& TestOpts :: new () , false , TestId (0) , desc , RunStrategy :: InProcess , tx) ; let result = rx . recv () . unwrap () . result ; assert_ne ! (result , TrOk) ; }
};
}
