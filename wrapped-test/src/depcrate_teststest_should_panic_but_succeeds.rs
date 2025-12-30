// Generated macro for test_should_panic_but_succeeds (function)
macro_rules! Depcrate_teststest_should_panic_but_succeeds {
() => {
// Module: crate::tests
// Provides: {"test_should_panic_but_succeeds"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore = "test requires unwinding support")] fn test_should_panic_but_succeeds () { let should_panic_variants = [ShouldPanic :: Yes , ShouldPanic :: YesWithMessage ("error message")] ; for & should_panic in should_panic_variants . iter () { fn f () -> Result < () , String > { Ok (()) } let desc = TestDescAndFn { desc : TestDesc { name : StaticTestName ("whatever") , ignore : false , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic , compile_fail : false , no_run : false , test_type : TestType :: Unknown , } , testfn : DynTestFn (Box :: new (f)) , } ; let (tx , rx) = channel () ; run_test (& TestOpts :: new () , false , TestId (0) , desc , RunStrategy :: InProcess , tx) ; let result = rx . recv () . unwrap () . result ; assert_eq ! (result , TrFailedMsg ("test did not panic as expected" . to_string ()) , "should_panic == {:?}" , should_panic) ; } }
};
}
