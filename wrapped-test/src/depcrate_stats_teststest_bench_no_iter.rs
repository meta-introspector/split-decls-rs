// Generated macro for test_bench_no_iter (function)
macro_rules! Depcrate_stats_teststest_bench_no_iter {
() => {
// Module: crate::stats::tests
// Provides: {"test_bench_no_iter"}
// Dependencies: {}
# [test] fn test_bench_no_iter () { fn f (_ : & mut Bencher) -> Result < () , String > { Ok (()) } let (tx , rx) = channel () ; let desc = TestDesc { name : StaticTestName ("f") , ignore : false , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic : ShouldPanic :: No , compile_fail : false , no_run : false , test_type : TestType :: Unknown , } ; crate :: bench :: benchmark (TestId (0) , desc , tx , true , f) ; rx . recv () . unwrap () ; }
};
}
