// Generated macro for test_should_panic_good_message (function)
macro_rules! Depcrate_teststest_should_panic_good_message {
() => {
// Module: crate::tests
// Provides: {"test_should_panic_good_message"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore = "test requires unwinding support")] fn test_should_panic_good_message () { fn f () -> Result < () , String > { panic ! ("an error message") ; } let desc = TestDescAndFn { desc : TestDesc { name : StaticTestName ("whatever") , ignore : false , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic : ShouldPanic :: YesWithMessage ("error message") , compile_fail : false , no_run : false , test_type : TestType :: Unknown , } , testfn : DynTestFn (Box :: new (f)) , } ; let (tx , rx) = channel () ; run_test (& TestOpts :: new () , false , TestId (0) , desc , RunStrategy :: InProcess , tx) ; let result = rx . recv () . unwrap () . result ; assert_eq ! (result , TrOk) ; }
};
}
