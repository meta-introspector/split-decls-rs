// Generated macro for test_should_panic_bad_message (function)
macro_rules! Depcrate_stats_teststest_should_panic_bad_message {
() => {
// Module: crate::stats::tests
// Provides: {"test_should_panic_bad_message"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore = "test requires unwinding support")] fn test_should_panic_bad_message () { use crate :: tests :: TrFailedMsg ; fn f () -> Result < () , String > { panic ! ("an error message") ; } let expected = "foobar" ; let failed_msg = r#"panic did not contain expected string
      panic message: "an error message"
 expected substring: "foobar""# ; let desc = TestDescAndFn { desc : TestDesc { name : StaticTestName ("whatever") , ignore : false , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic : ShouldPanic :: YesWithMessage (expected) , compile_fail : false , no_run : false , test_type : TestType :: Unknown , } , testfn : DynTestFn (Box :: new (f)) , } ; let (tx , rx) = channel () ; run_test (& TestOpts :: new () , false , TestId (0) , desc , RunStrategy :: InProcess , tx) ; let result = rx . recv () . unwrap () . result ; assert_eq ! (result , TrFailedMsg (failed_msg . to_string ())) ; }
};
}
