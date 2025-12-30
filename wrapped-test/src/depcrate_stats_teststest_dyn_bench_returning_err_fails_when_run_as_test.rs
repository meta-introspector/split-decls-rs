// Generated macro for test_dyn_bench_returning_err_fails_when_run_as_test (function)
macro_rules! Depcrate_stats_teststest_dyn_bench_returning_err_fails_when_run_as_test {
() => {
// Module: crate::stats::tests
// Provides: {"test_dyn_bench_returning_err_fails_when_run_as_test"}
// Dependencies: {}
# [test] # [cfg (not (target_os = "emscripten"))] fn test_dyn_bench_returning_err_fails_when_run_as_test () { fn f (_ : & mut Bencher) -> Result < () , String > { Result :: Err ("An error" . into ()) } let desc = TestDescAndFn { desc : TestDesc { name : StaticTestName ("whatever") , ignore : false , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic : ShouldPanic :: No , compile_fail : false , no_run : false , test_type : TestType :: Unknown , } , testfn : DynBenchFn (Box :: new (f)) , } ; let (tx , rx) = channel () ; let notify = move | event : TestEvent | { if let TestEvent :: TeResult (result) = event { tx . send (result) . unwrap () ; } Ok (()) } ; run_tests (& TestOpts { run_tests : true , .. TestOpts :: new () } , vec ! [desc] , notify) . unwrap () ; let result = rx . recv () . unwrap () . result ; assert_eq ! (result , TrFailed) ; }
};
}
