// Generated macro for report_time_test_template (function)
macro_rules! Depcrate_testsreport_time_test_template {
() => {
// Module: crate::tests
// Provides: {"report_time_test_template"}
// Dependencies: {}
fn report_time_test_template (report_time : bool) -> Option < TestExecTime > { fn f () -> Result < () , String > { Ok (()) } let desc = TestDescAndFn { desc : TestDesc { name : StaticTestName ("whatever") , ignore : false , ignore_message : None , source_file : "" , start_line : 0 , start_col : 0 , end_line : 0 , end_col : 0 , should_panic : ShouldPanic :: No , compile_fail : false , no_run : false , test_type : TestType :: Unknown , } , testfn : DynTestFn (Box :: new (f)) , } ; let time_options = if report_time { Some (TestTimeOptions :: default ()) } else { None } ; let test_opts = TestOpts { time_options , .. TestOpts :: new () } ; let (tx , rx) = channel () ; run_test (& test_opts , false , TestId (0) , desc , RunStrategy :: InProcess , tx) ; let exec_time = rx . recv () . unwrap () . exec_time ; exec_time }
};
}
