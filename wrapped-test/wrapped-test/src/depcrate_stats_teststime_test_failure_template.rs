// Generated macro for time_test_failure_template (function)
macro_rules! Depcrate_stats_teststime_test_failure_template {
() => {
// Module: crate::stats::tests
// Provides: {"time_test_failure_template"}
// Dependencies: {}
fn time_test_failure_template (test_type : TestType) -> TestResult { fn f () -> Result < () , String > { Ok (()) } let desc = TestDescAndFn { desc : TestDesc { name : StaticTestName ("whatever") , ignore : false , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic : ShouldPanic :: No , compile_fail : false , no_run : false , test_type , } , testfn : DynTestFn (Box :: new (f)) , } ; let mut time_options = TestTimeOptions :: default () ; time_options . error_on_excess = true ; let test_opts = TestOpts { time_options : Some (time_options) , .. TestOpts :: new () } ; let (tx , rx) = channel () ; run_test (& test_opts , false , TestId (0) , desc , RunStrategy :: InProcess , tx) ; let result = rx . recv () . unwrap () . result ; result }
};
}
