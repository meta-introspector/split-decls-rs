// Generated macro for spawn_test_subprocess (function)
macro_rules! Depcratespawn_test_subprocess {
() => {
// Module: crate
// Provides: {"spawn_test_subprocess"}
// Dependencies: {}
fn spawn_test_subprocess (id : TestId , desc : TestDesc , nocapture : bool , report_time : bool , monitor_ch : Sender < CompletedTest > , time_opts : Option < time :: TestTimeOptions > , bench_benchmarks : bool ,) { let (result , test_output , exec_time) = (| | { let args = env :: args () . collect :: < Vec < _ > > () ; let current_exe = & args [0] ; let mut command = Command :: new (current_exe) ; command . env (SECONDARY_TEST_INVOKER_VAR , desc . name . as_slice ()) ; if bench_benchmarks { command . env (SECONDARY_TEST_BENCH_BENCHMARKS_VAR , "1") ; } if nocapture { command . stdout (process :: Stdio :: inherit ()) ; command . stderr (process :: Stdio :: inherit ()) ; } let start = report_time . then (Instant :: now) ; let output = match command . output () { Ok (out) => out , Err (e) => { let err = format ! ("Failed to spawn {} as child for test: {:?}" , args [0] , e) ; return (TrFailed , err . into_bytes () , None) ; } } ; let exec_time = start . map (| start | { let duration = start . elapsed () ; TestExecTime (duration) }) ; let std :: process :: Output { stdout , stderr , status } = output ; let mut test_output = stdout ; formatters :: write_stderr_delimiter (& mut test_output , & desc . name) ; test_output . extend_from_slice (& stderr) ; let result = get_result_from_exit_code (& desc , status , time_opts . as_ref () , exec_time . as_ref ()) ; (result , test_output , exec_time) }) () ; let message = CompletedTest :: new (id , desc , result , exec_time , test_output) ; monitor_ch . send (message) . unwrap () ; }
};
}
